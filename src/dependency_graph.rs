use std::collections::HashMap;

#[derive(Debug)]
pub struct ComponentIdentity {
    pub file_path: String,
    pub component_name: String,
}

impl ComponentIdentity {
    pub fn new(file_path: String, component_name: String) -> Self {
        Self {
            file_path,
            component_name,
        }
    }
}

#[derive(Debug)]
pub struct DependencyGraph {
    pub data: HashMap<String, HashMap<String, Vec<ComponentIdentity>>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, current: ComponentIdentity, depend_on: ComponentIdentity) {
        let dependency_list =
            self.get_dependency_list(depend_on.file_path, depend_on.component_name);

        // avoid duplicate dependency by comparing the file path and component name
        if dependency_list
            .iter()
            .any(|x| x.file_path == current.file_path && x.component_name == current.component_name)
        {
            return;
        }

        dependency_list.push(current);
    }

    fn get_component_map(
        &mut self,
        file_path: String,
    ) -> &mut HashMap<String, Vec<ComponentIdentity>> {
        self.data.entry(file_path).or_insert(HashMap::new())
    }

    fn get_dependency_list(
        &mut self,
        file_path: String,
        component_name: String,
    ) -> &mut Vec<ComponentIdentity> {
        let component_map = self.get_component_map(file_path);

        component_map.entry(component_name).or_insert(Vec::new())
    }
}
