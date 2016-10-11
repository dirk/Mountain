pub trait Menu {
    type MenuItem;

    fn new(String, items: Option<Vec<Self::MenuItem>>) -> Self;
}
