use std::io;
// 키와 몸무게 데이터를 가지는 구조체 --- (*1)
struct Body {
    weight: f64,
    height: f64,
    name: String,
}

impl Body { 
    fn BMI(&self) -> f64 {
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }

    fn new(weight: f64, height: f64, name: String) -> Body {
        Body {
          weight,
          height,
          name,
        }
      }
}

impl Body {
    fn show(&self) {
      print!("{}! {}kg and {}cm with BMI: {:.2}.", self.name, self.weight, self.height, self.BMI());
    }
}

fn input(prompt: &str) -> f64 {
  println!("{}", prompt);
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).expect("Failed to read line");
  s.trim().parse().expect("Not a Number.")
}

fn main() {
    // 구조체 초기화
    println!("Type your name: ");
    let mut naming = String::new();
    io::stdin().read_line(&mut naming).expect("Failed to read line");
    let naming = naming.trim().to_string();

    
    let w1 = input("Type your weight: ");
    let h1 = input("Type your height: ");
    
    let lim = Body {
        weight: w1,
        height: h1,
        name: naming,
    };

    lim.show();
}
