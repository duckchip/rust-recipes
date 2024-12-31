CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    description TEXT,
    ingredients JSON,
    instructions TEXT,  
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO recipes (name, description, ingredients, instructions)
VALUES
    ('Spaghetti Carbonara', 'A classic Italian pasta dish made with eggs, cheese, pancetta, and pepper.', 
        '["spaghetti", "eggs", "pancetta", "Parmesan cheese", "black pepper"]', 
        'Cook pasta. Fry pancetta until crispy. Mix eggs and cheese. Combine pasta, pancetta, and egg mixture.'),
    
    ('Chicken Alfredo', 'A creamy pasta dish with chicken, garlic, and Parmesan cheese.', 
        '["chicken breast", "fettuccine", "garlic", "butter", "heavy cream", "Parmesan cheese"]', 
        'Cook pasta. Sauté chicken with garlic. Add butter, cream, and cheese to make the sauce.'),
    
    ('Vegetable Stir Fry', 'A quick and healthy vegetable stir fry with soy sauce and sesame oil.',
        '["broccoli", "carrots", "bell peppers", "soy sauce", "sesame oil", "garlic"]', 
        'Stir-fry vegetables with soy sauce and sesame oil. Serve with rice or noodles.'),
    
    ('Beef Tacos', 'A Tex-Mex favorite made with ground beef, cheese, and tortillas.',
        '["ground beef", "taco seasoning", "tortillas", "lettuce", "cheese", "sour cream"]', 
        'Cook beef with seasoning. Fill tortillas with beef and toppings.'),
    
    ('Margherita Pizza', 'A simple pizza topped with fresh tomatoes, mozzarella, and basil.',
        '["pizza dough", "tomatoes", "mozzarella cheese", "fresh basil", "olive oil"]', 
        'Bake pizza dough. Top with tomatoes, cheese, and basil. Bake again until golden.'),
    
    ('Caesar Salad', 'A classic salad with Romaine lettuce, croutons, Parmesan cheese, and Caesar dressing.',
        '["romaine lettuce", "croutons", "Parmesan cheese", "Caesar dressing"]', 
        'Toss lettuce with dressing and top with croutons and Parmesan cheese.'),
    
    ('Chicken Parmesan', 'Breaded chicken cutlets topped with marinara sauce and melted cheese.',
        '["chicken breasts", "bread crumbs", "marinara sauce", "mozzarella cheese", "Parmesan cheese"]', 
        'Bread chicken, fry until golden. Top with sauce and cheese, bake until cheese melts.'),
    
    ('Grilled Salmon', 'A healthy and flavorful grilled salmon fillet served with a lemon dill sauce.',
        '["salmon", "lemon", "dill", "olive oil", "garlic"]', 
        'Grill salmon and serve with a sauce made of lemon, dill, and olive oil.'),
    
    ('Mushroom Risotto', 'A creamy rice dish made with mushrooms, Parmesan, and white wine.',
        '["rice", "mushrooms", "white wine", "Parmesan cheese", "butter"]', 
        'Cook rice with mushrooms and wine. Stir in Parmesan cheese and butter.'),
    
    ('Peking Duck', 'A Chinese dish featuring crispy duck served with pancakes, hoisin sauce, and vegetables.',
        '["duck", "hoisin sauce", "pancakes", "cucumber", "scallions"]', 
        'Roast duck until crispy. Serve with pancakes and vegetables, drizzle with hoisin sauce.'),
    
    ('Beef Wellington', 'A luxurious beef dish wrapped in puff pastry with mushroom duxelles.',
        '["beef tenderloin", "puff pastry", "mushrooms", "foie gras", "egg"]', 
        'Sear beef, wrap in mushrooms and pastry, bake until golden.'),
    
    ('Vegetarian Chili', 'A hearty vegetarian chili with beans, tomatoes, and spices.',
        '["beans", "tomatoes", "bell peppers", "chili powder", "onions", "garlic"]', 
        'Cook beans and vegetables with spices. Simmer until thickened.'),
    
    ('Shrimp Scampi', 'Shrimp cooked in a garlic butter sauce served with pasta.',
        '["shrimp", "garlic", "butter", "lemon", "pasta", "parsley"]', 
        'Sauté shrimp with garlic and butter. Toss with pasta and lemon juice.'),
    
    ('Lamb Curry', 'A flavorful curry made with lamb and aromatic spices.',
        '["lamb", "curry powder", "garlic", "ginger", "tomatoes", "coconut milk"]', 
        'Cook lamb with spices, add coconut milk, and simmer until tender.'),
    
    ('Pulled Pork Sandwiches', 'Slow-cooked pork with barbecue sauce served on buns.',
        '["pork shoulder", "BBQ sauce", "buns", "coleslaw"]', 
        'Cook pork until tender. Shred and serve with BBQ sauce and coleslaw on buns.'),
    
    ('Fettuccine Alfredo', 'A creamy pasta dish with fettuccine, heavy cream, and Parmesan.',
        '["fettuccine", "heavy cream", "garlic", "butter", "Parmesan cheese"]', 
        'Cook pasta. Make creamy sauce with cream, butter, and cheese. Combine.'),
    
    ('Beef Stew', 'A hearty stew made with beef, carrots, potatoes, and broth.',
        '["beef", "carrots", "potatoes", "onions", "beef broth", "garlic"]', 
        'Simmer beef and vegetables in broth until tender.'),
    
    ('Chicken Tikka Masala', 'A popular Indian dish made with chicken in a creamy spiced tomato sauce.',
        '["chicken", "tomatoes", "heavy cream", "curry powder", "garlic", "ginger"]', 
        'Cook chicken in spiced sauce made with tomatoes and cream.'),
    
    ('Fish and Chips', 'Crispy fried fish served with fries and tartar sauce.',
        '["white fish", "potatoes", "flour", "beer", "tartar sauce"]', 
        'Fry battered fish and potatoes. Serve with tartar sauce.'),
    
    ('Pasta Primavera', 'A fresh vegetable pasta dish with garlic and olive oil.',
        '["pasta", "bell peppers", "zucchini", "tomatoes", "garlic", "olive oil"]', 
        'Cook pasta and vegetables. Toss with garlic and olive oil.'),
    
    ('Chicken Wings', 'Spicy chicken wings served with a side of blue cheese dressing.',
        '["chicken wings", "hot sauce", "butter", "blue cheese dressing"]', 
        'Toss wings in hot sauce and bake. Serve with dressing.'),
    
    ('Lasagna', 'Layers of pasta, ground beef, cheese, and marinara sauce baked together.',
        '["lasagna noodles", "ground beef", "ricotta cheese", "mozzarella", "marinara sauce"]', 
        'Layer pasta, meat, cheese, and sauce. Bake until bubbly.'),
    
    ('Eggplant Parmesan', 'Breaded eggplant slices topped with marinara sauce and mozzarella.',
        '["eggplant", "bread crumbs", "marinara sauce", "mozzarella cheese"]', 
        'Bread eggplant and fry. Top with sauce and cheese, bake until melted.'),
    
    ('Pasta Bolognese', 'A rich meat sauce served over pasta.',
        '["ground beef", "tomatoes", "garlic", "onion", "pasta"]', 
        'Cook ground beef and vegetables. Add tomatoes and simmer. Serve with pasta.'),
    
    ('Chili Con Carne', 'A hearty beef chili with beans, tomatoes, and chili spices.',
        '["ground beef", "beans", "tomatoes", "chili powder", "onions"]', 
        'Cook beef and vegetables, add spices and simmer until thickened.'),
    
    ('Chicken Soup', 'A comforting soup made with chicken, vegetables, and broth.',
        '["chicken", "carrots", "celery", "onions", "chicken broth"]', 
        'Simmer chicken with vegetables and broth. Serve hot.'),
    
    ('BBQ Ribs', 'Slow-cooked ribs with barbecue sauce.',
        '["ribs", "BBQ sauce", "garlic", "brown sugar"]', 
        'Cook ribs low and slow. Baste with BBQ sauce until caramelized.'),
    
    ('Steak Frites', 'A simple dish of steak served with crispy fries.',
        '["steak", "potatoes", "garlic", "butter"]', 
        'Grill steak to desired doneness. Serve with fries.'),
    
    ('Pumpkin Soup', 'A smooth and creamy soup made with pureed pumpkin.',
        '["pumpkin", "onions", "garlic", "cream", "vegetable broth"]', 
        'Cook pumpkin with onions and garlic, blend until smooth.'),
    
    ('Shrimp Fried Rice', 'A quick stir-fried rice dish with shrimp and vegetables.',
        '["shrimp", "rice", "peas", "carrots", "soy sauce", "garlic"]', 
        'Fry rice with shrimp and vegetables. Add soy sauce for flavor.')
;