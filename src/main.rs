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

/// Compose two functions (for demonstrating category theory laws)
/// Composes functions g ∘ f, creating a new function that applies f then g
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    // Demonstrate the identity function with different types
    
    // Identity on integers
    let x = 42;
    assert_eq!(id(x), x);
    println!("id({}) = {}", x, id(x));
    
    // Identity on strings
    let s = String::from("hello");
    let s_result = id(s.clone());
    assert_eq!(s, s_result);
    println!("id(\"{}\") = \"{}\"", s, s_result);
    
    // Identity on vectors
    let v = vec![1, 2, 3];
    let v_result = id(v.clone());
    assert_eq!(v, v_result);
    println!("id({:?}) = {:?}", v, v_result);
    
    // Demonstrate identity laws
    println!("\nDemonstrating category theory identity laws:");
    
    // Some example functions
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    
    // Left identity: id ∘ f = f
    let left_identity = compose(add_one, id);
    let test_val = 5;
    assert_eq!(left_identity(test_val), add_one(test_val));
    println!("Left identity: (id ∘ add_one)({}) = add_one({}) = {}", 
             test_val, test_val, left_identity(test_val));
    
    // Right identity: f ∘ id = f
    let right_identity = compose(id, double);
    assert_eq!(right_identity(test_val), double(test_val));
    println!("Right identity: (double ∘ id)({}) = double({}) = {}", 
             test_val, test_val, right_identity(test_val));
    
    println!("\nIdentity function implementation successful!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
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