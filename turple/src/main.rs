fn main() {
    let text = "i see the eigenvalue in thine eye";
    let (head,tail) = text.split_at(21);
    assert_eq!(head,"i see the eigenvalue " );
    assert_eq!(tail,"in thine eye");
}
