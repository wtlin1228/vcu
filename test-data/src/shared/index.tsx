// src/shared.js

// `src/shared.js:Header` is used by 
// [
//     { path: "src/shared.js", component: "Layout" },
//     { path: "src/step.js", component: "Step" },
//     { path: "src/shuffle.js", component: "Default" },
// ]
export { Header } from './Header'

// `src/shared.js:Body` is used by 
// [
//     { path: "src/shared.js", component: "Layout" },
// ]
export const Body = () => <div>Body</div>   

// `src/shared.js:Footer` is used by 
// [
//     { path: "src/shared.js", component: "Layout" },
//     { path: "src/shared.js", component: "Steps" },
// ]
export const Footer = () => <div>Footer</div>

export const Layout = () => (
    <>
        <Header />
        <Body />
        <Footer />
    </>
)

export default Layout