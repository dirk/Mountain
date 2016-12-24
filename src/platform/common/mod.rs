pub trait Menu {
    type MenuItem;

    fn new(String, Option<Vec<Self::MenuItem>>) -> Self;
}
