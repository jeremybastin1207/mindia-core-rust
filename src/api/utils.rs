pub(crate) fn parse_transformation_from_path(path: &str) -> (String, String) {
    let mut transformation_chain = String::new();
    let mut image_path = String::new();

    for part in path.split("/") {
        if part.starts_with("t_") || part.starts_with("c_") {
            transformation_chain.push_str(part);
            transformation_chain.push_str("/");
        } else {
            image_path.push_str(part);
            image_path.push_str("/");
        }
    }

    transformation_chain.pop();
    image_path.pop();

    (transformation_chain, image_path)
}
