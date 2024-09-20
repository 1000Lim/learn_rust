trait TreasureBox {
    fn open(&self, key_no: i32) -> bool {
        // define a default method
        self.get_key_no() == key_no
    }
    fn check(&self); // check the box
    fn get_key_no(&self) -> i32;
}

struct JewelryBox {
    price: i32,
    key_no: i32,
}

struct EmptyBox {
    key_no: i32,
}

impl TreasureBox for JewelryBox {
    fn open(&self, key_no: i32) -> bool {
        self.key_no == key_no
    }

    fn check(&self) {
        println!("You opened a jewelry box and get the price: {}", self.price);
    }

    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("You opened an empty box.");
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("You can't open the box.");
        return;
    }
    tbox.check();
}

fn main() {
    let box1 = JewelryBox {
        price: 1000,
        key_no: 1,
    };

    let box2 = JewelryBox {
        price: 2000,
        key_no: 2,
    };

    let box3 = EmptyBox { key_no: 1 };

    let my_key = 1;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}
