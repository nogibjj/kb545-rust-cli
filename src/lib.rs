use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ];

    let mut rng = thread_rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}

pub fn add_veggies(fruits: Vec<String>, num_veggies: usize) -> Vec<String> {

    let mut new_fruits = fruits.clone();

    let veggies = vec![
        "Broccoli".to_string(),
        "Carrot".to_string(),
        "Cauliflower".to_string(),
        "Celery".to_string(),
        "Cucumber".to_string(),
        "Lettuce".to_string(),
        "Mushroom".to_string(),
        "Onion".to_string(),
    ];

    let mut rng = thread_rng();
    let mut veggies = veggies;
    veggies.shuffle(&mut rng);

    for item in veggies.into_iter().take(num_veggies) {
        new_fruits.push(item);
    }

    new_fruits
}


#[test]
fn test_both_functions(){
    let num_fruits = 3;
    let num_veggies = 2;
    let fruit_salad = create_fruit_salad(num_fruits);
    assert_eq!(fruit_salad.len(), num_fruits);
    let salad = add_veggies(fruit_salad, num_veggies);
    assert_eq!(salad.len(), num_fruits+num_veggies);
}