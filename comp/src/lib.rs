#[cfg(test)]
mod tests {
    use comp_macro::comp;

    #[test]
    fn it_works() {
        let result = comp![x for x in [1,2,3]].collect::<Vec<_>>();
        assert_eq!(result, [1, 2, 3]);

        let result = comp![x*2 for x in [1,2,3] if x %2 == 0].collect::<Vec<_>>();
        assert_eq!(result, [4]);
    }
}
