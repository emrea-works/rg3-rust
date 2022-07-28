struct Actor {
    first_name: String,
    last_name: String,
    insta: String,
    birthyear: Date,
    height: f32,
    hair: String,
    eyes: String,
    education: String,
    series: String,
    cinema: String,
    tv: String,
    thatre: String,
    awards: String,
    more: String,
}

impl Actor {
    // Construct
    fn new(
        first_n: &str, 
        last_n: &str,
        birthyear: &date,  
        height: &str,
        hair: &str,
        eyes: &str,
        education: &str,
        series: &str,
        cinema: &str,
        tv: &str,
        thatre: &str,
        awards: &str,
        mor:&str,
    ) -> Actor {
        Actor {
            first_name: first_n.to_string(),
            last_name: last_n.to_string(),
            birthyear:  
            height: 
            hair: 
            eyes: 
            education: 
            series: 
            cinema: 
            tv: 
            thatre: 
            awards: 
            mor: 
        }
    }
    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // Set first name
    fn set_first_name(&mut self, first_n: &str) {
        self.first_name = first_n.to_string();
    }
    // Set last name
    fn set_last_name(&mut self, last_n: &str) {
        self.last_name = last_n.to_string();
    }
    // Set social media accounts
    fn set_social_media(&mut self, s_m_a: &str) {
        self.insta = s_m_a.to_string();
    }
    // Set birthyear
    // Set height
    // Set hair
    // Set eyes
    // Set education
    // Set series
    // Set cinema
    // Set tv
    // Set thatre
    // Set awards
    // Set mor

    // Get tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut a = Actor::new("Ayşenil", "Şamlıoğlu");
    println!("{}", a.full_name());
}
