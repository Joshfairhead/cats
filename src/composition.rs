/// Function composition from category theory.
///
/// In category theory, composition is a fundamental operation that combines
/// two morphisms (arrows) to create a new morphism.
///
/// Given:
/// - f: A → B
/// - g: B → C
///
/// The composition g ∘ f: A → C is defined as (g ∘ f)(x) = g(f(x))
///
/// Composition must satisfy the associativity law:
/// h ∘ (g ∘ f) = (h ∘ g) ∘ f
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::id;
    
    #[test]
    fn test_basic_composition() {
        let add_one = |x: i32| x + 1;
        let multiply_two = |x: i32| x * 2;
        
        // (x + 1) * 2
        let composed = compose(add_one, multiply_two);
        assert_eq!(composed(5), 12); // (5 + 1) * 2 = 12
    }
    
    #[test]
    fn test_associativity_law() {
        let f = |x: i32| x + 1;
        let g = |x: i32| x * 2;
        let h = |x: i32| x - 3;
        
        // h ∘ (g ∘ f)
        let left_assoc = compose(compose(f, g), h);
        
        // (h ∘ g) ∘ f
        let right_assoc = compose(f, compose(g, h));
        
        for x in -10..10 {
            assert_eq!(left_assoc(x), right_assoc(x));
        }
    }
    
    #[test]
    fn test_composition_with_identity() {
        let f = |x: i32| x * 3;
        
        // id ∘ f = f
        let left_id = compose(f, id);
        // f ∘ id = f
        let right_id = compose(id, f);
        
        for x in -10..10 {
            assert_eq!(left_id(x), f(x));
            assert_eq!(right_id(x), f(x));
        }
    }
    
    #[test]
    fn test_string_composition() {
        let to_uppercase = |s: String| s.to_uppercase();
        let add_exclamation = |s: String| format!("{}!", s);
        
        let composed = compose(to_uppercase, add_exclamation);
        assert_eq!(composed("hello".to_string()), "HELLO!");
    }
}