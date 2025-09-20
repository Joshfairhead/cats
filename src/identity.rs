/// The identity function from category theory.
/// 
/// In category theory, the identity morphism id_A : A → A
/// is the unique morphism that satisfies:
/// - Left identity: id_B ∘ f = f for any f: A → B
/// - Right identity: f ∘ id_A = f for any f: A → B
///
/// This is the polymorphic identity function that works for any type T.
pub fn id<T>(x: T) -> T {
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition::compose;
    
    #[test]
    fn test_identity_preserves_value() {
        assert_eq!(id(42), 42);
        assert_eq!(id("hello"), "hello");
        assert_eq!(id(vec![1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(id(true), true);
        assert_eq!(id(3.14), 3.14);
    }
    
    #[test]
    fn test_left_identity_law() {
        let f = |x: i32| x * 2;
        let composed = compose(id, f);
        
        for x in -10..10 {
            assert_eq!(composed(x), f(x));
        }
    }
    
    #[test]
    fn test_right_identity_law() {
        let f = |x: i32| x + 10;
        let composed = compose(f, id);
        
        for x in -10..10 {
            assert_eq!(composed(x), f(x));
        }
    }
    
    #[test]
    fn test_identity_composition() {
        // id ∘ id = id
        let double_id = compose(id::<i32>, id::<i32>);
        
        for x in -10..10 {
            assert_eq!(double_id(x), id(x));
            assert_eq!(double_id(x), x);
        }
    }
}