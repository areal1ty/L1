pub fn remove_element_at<T>(vec: &mut Vec<T>, index: usize) -> Option<T> {
    if index < vec.len() {
        Some(vec.remove(index))
    } else {
        None
    }
}