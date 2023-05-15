type Monster = (String, u32);

fn main() {
    let m: Monster = ("Ultra Sunrise".to_string(), 355);
    {
        let r: &Monster = &m;
        ggg(r);
    }
}

fn ggg(monster: &Monster) {
    println!("몬스터 에너지: {} / 캔 {} ml", monster.0, monster.1);
}

