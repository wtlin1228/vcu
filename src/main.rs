use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use swc_common::{
    errors::{ColorConfig, Handler},
    input::StringInput,
    sync::Lrc,
    SourceMap,
};
use swc_ecma_ast::*;
use swc_ecma_parser::{lexer::Lexer, Parser, Syntax, TsConfig};
use swc_ecma_visit::{Visit, VisitWith};

mod dependency_graph;
use dependency_graph::{ComponentIdentity, DependencyGraph};

struct DependencyVisitor {
    pub graph: DependencyGraph,
    project_root: PathBuf,
    current_file: PathBuf,
    current_imports: HashMap<String, ComponentIdentity>,
}

impl DependencyVisitor {
    fn new(project_root: &str) -> Self {
        Self {
            graph: DependencyGraph::new(),
            project_root: PathBuf::from(project_root),
            current_file: PathBuf::new(),
            current_imports: HashMap::new(),
        }
    }

    fn resolve_module_path(&self, module_path: &str) -> PathBuf {
        let is_relative = module_path.starts_with('.');
        let absolute_path = match is_relative {
            true => {
                let mut res = self.current_file.clone();
                res.pop();
                res.push(module_path);
                res.canonicalize().unwrap()
            }
            false => {
                let mut res = self.project_root.clone();
                res.push(module_path);
                res
            }
        };

        // Resolve the file extension
        let supported_extensions = ["tsx", "jsx", "ts", "js"];
        for ext in supported_extensions.iter() {
            let mut candidate = absolute_path.clone();
            candidate.set_extension(ext);
            if candidate.exists() {
                return candidate;
            }
        }

        // If no file is found with supported_extensions, check for index.* files in the folder
        for ext in supported_extensions.iter() {
            let mut candidate = absolute_path.clone();
            candidate.push("index");
            candidate.set_extension(ext);
            if candidate.exists() {
                return candidate;
            }
        }

        panic!("Module not found: {}", module_path);
    }

    fn before_process_file(&mut self, file_path: &Path) {
        self.current_file = file_path.to_path_buf();
        self.current_imports.clear();
    }

    fn process_file(&mut self, file_path: &Path) {
        self.before_process_file(file_path);

        let cm: Lrc<SourceMap> = Default::default();
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

        let fm = cm.load_file(file_path).expect("failed to load the file");
        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                ..Default::default()
            }),
            EsVersion::latest(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        let m = parser
            .parse_module()
            .map_err(|e| e.into_diagnostic(&handler).emit())
            .expect("failed to parse module");

        m.visit_with(&mut *self);
    }
}

impl Visit for DependencyVisitor {
    fn visit_import_decl(&mut self, import_decl: &ImportDecl) {
        let module_path = import_decl.src.value.to_string();
        let resolved_path = self.resolve_module_path(&module_path);

        for specifier in &import_decl.specifiers {
            match specifier {
                swc_ecma_ast::ImportSpecifier::Default(default_specifier) => {
                    let local_ident = &default_specifier.local.sym;
                    self.current_imports.insert(
                        local_ident.to_string(),
                        ComponentIdentity::new(
                            resolved_path.to_string_lossy().to_string(),
                            "Default".to_string(),
                        ),
                    );
                }
                swc_ecma_ast::ImportSpecifier::Named(named_specifier) => {
                    let local_ident = &named_specifier.local.sym;
                    let imported_ident = named_specifier
                        .imported
                        .as_ref()
                        .map(|id| match id {
                            swc_ecma_ast::ModuleExportName::Ident(ident) => ident.sym.to_string(),
                            _ => local_ident.to_string(),
                        })
                        .unwrap_or_else(|| local_ident.to_string());
                    self.current_imports.insert(
                        local_ident.to_string(),
                        ComponentIdentity::new(
                            resolved_path.to_string_lossy().to_string(),
                            imported_ident,
                        ),
                    );
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let mut visitor = DependencyVisitor::new("<Input the root of your project here>");

    // TODO: process all files in the project
    visitor.process_file(Path::new(
        "<Input the path of the file you want to analyze here>",
    ));

    println!("{:#?}", visitor.current_imports);
}
