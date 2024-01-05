use num_format::{Locale, ToFormattedString};
use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use std::time::Instant;

fn main() {
    let cmd = std::env::args().nth(1).expect("command");

    match cmd.as_ref() {
        "create_measurements" => create_measurements(),
        _ => calculate(),
    }
}

struct Nums {
    pub count: u64,
    pub min: f64,
    pub mean: f64,
    pub max: f64,
}

impl Nums {
    pub fn update(&mut self, num: f64) {
        self.count += 1;
        self.mean += (num - self.mean) / (self.count as f64);

        if num < self.min {
            self.min = num;
        }

        if num > self.max {
            self.max = num;
        }
    }
}

fn calculate() {
    let mut nums: HashMap<String, Nums> = HashMap::new();
    let reader = BufReader::new(File::open("measurements.txt").expect("read"));
    let start = Instant::now();

    for (i, line) in reader.lines().enumerate() {
        if i > 0 && i % 50_000_000 == 0 {
            println!(
                "Wrote {} measurements in {:?}",
                i.to_formatted_string(&Locale::en),
                start.elapsed()
            );
        }

        if let Ok(line) = line {
            let (id, num) = line.split_once(';').unwrap();
            let num: f64 = num.parse().expect("float");

            if let Some(n) = nums.get_mut(id) {
                n.update(num);
            } else {
                nums.insert(
                    id.to_string(),
                    Nums {
                        count: 1,
                        min: num,
                        mean: num,
                        max: num,
                    },
                );
            }
        }
    }

    for (id, n) in nums.iter() {
        println!(
            "{}={}/{}/{}/{}",
            id,
            n.count,
            n.min,
            (n.mean * 10.0).round() / 10.0,
            n.max
        );
    }
}

fn create_measurements() {
    let size: u64 = std::env::args()
        .nth(2)
        .expect("size")
        .parse()
        .expect("number");
    let stations = WeatherStation::list();
    let mut rng = thread_rng();
    let mut f = BufWriter::new(
        File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open("measurements.txt")
            .unwrap(),
    );

    let start = Instant::now();

    for i in 1..size + 1 {
        if i % 50_000_000 == 0 {
            println!(
                "Wrote {} measurements in {:?}",
                i.to_formatted_string(&Locale::en),
                start.elapsed()
            );
        }

        let station = stations.choose(&mut rng).unwrap();
        writeln!(&mut f, "{};{}", station.id(), station.measurement(&mut rng)).expect("write");
    }
}

#[derive(Debug)]
struct WeatherStation(String, f64);

impl WeatherStation {
    pub fn id(&self) -> &str {
        &self.0
    }

    pub fn measurement(&self, rng: &mut ThreadRng) -> f64 {
        let normal = Normal::new(self.1, 10.0).unwrap();
        let m: f64 = normal.sample(rng);
        (m * 10.0).round() / 10.0
    }

    pub fn list() -> Vec<Self> {
        vec![
            WeatherStation("Abha".to_string(), 18.0),
            WeatherStation("Abidjan".to_string(), 26.0),
            WeatherStation("Abéché".to_string(), 29.4),
            WeatherStation("Accra".to_string(), 26.4),
            WeatherStation("Addis Ababa".to_string(), 16.0),
            WeatherStation("Adelaide".to_string(), 17.3),
            WeatherStation("Aden".to_string(), 29.1),
            WeatherStation("Ahvaz".to_string(), 25.4),
            WeatherStation("Albuquerque".to_string(), 14.0),
            WeatherStation("Alexandra".to_string(), 11.0),
            WeatherStation("Alexandria".to_string(), 20.0),
            WeatherStation("Algiers".to_string(), 18.2),
            WeatherStation("Alice Springs".to_string(), 21.0),
            WeatherStation("Almaty".to_string(), 10.0),
            WeatherStation("Amsterdam".to_string(), 10.2),
            WeatherStation("Anadyr".to_string(), -6.9),
            WeatherStation("Anchorage".to_string(), 2.8),
            WeatherStation("Andorra la Vella".to_string(), 9.8),
            WeatherStation("Ankara".to_string(), 12.0),
            WeatherStation("Antananarivo".to_string(), 17.9),
            WeatherStation("Antsiranana".to_string(), 25.2),
            WeatherStation("Arkhangelsk".to_string(), 1.3),
            WeatherStation("Ashgabat".to_string(), 17.1),
            WeatherStation("Asmara".to_string(), 15.6),
            WeatherStation("Assab".to_string(), 30.5),
            WeatherStation("Astana".to_string(), 3.5),
            WeatherStation("Athens".to_string(), 19.2),
            WeatherStation("Atlanta".to_string(), 17.0),
            WeatherStation("Auckland".to_string(), 15.2),
            WeatherStation("Austin".to_string(), 20.7),
            WeatherStation("Baghdad".to_string(), 22.77),
            WeatherStation("Baguio".to_string(), 19.5),
            WeatherStation("Baku".to_string(), 15.1),
            WeatherStation("Baltimore".to_string(), 13.1),
            WeatherStation("Bamako".to_string(), 27.8),
            WeatherStation("Bangkok".to_string(), 28.6),
            WeatherStation("Bangui".to_string(), 26.0),
            WeatherStation("Banjul".to_string(), 26.0),
            WeatherStation("Barcelona".to_string(), 18.2),
            WeatherStation("Bata".to_string(), 25.1),
            WeatherStation("Batumi".to_string(), 14.0),
            WeatherStation("Beijing".to_string(), 12.9),
            WeatherStation("Beirut".to_string(), 20.9),
            WeatherStation("Belgrade".to_string(), 12.5),
            WeatherStation("Belize City".to_string(), 26.7),
            WeatherStation("Benghazi".to_string(), 19.9),
            WeatherStation("Bergen".to_string(), 7.7),
            WeatherStation("Berlin".to_string(), 10.3),
            WeatherStation("Bilbao".to_string(), 14.7),
            WeatherStation("Birao".to_string(), 26.5),
            WeatherStation("Bishkek".to_string(), 11.3),
            WeatherStation("Bissau".to_string(), 27.0),
            WeatherStation("Blantyre".to_string(), 22.2),
            WeatherStation("Bloemfontein".to_string(), 15.6),
            WeatherStation("Boise".to_string(), 11.4),
            WeatherStation("Bordeaux".to_string(), 14.2),
            WeatherStation("Bosaso".to_string(), 30.0),
            WeatherStation("Boston".to_string(), 10.9),
            WeatherStation("Bouaké".to_string(), 26.0),
            WeatherStation("Bratislava".to_string(), 10.5),
            WeatherStation("Brazzaville".to_string(), 25.0),
            WeatherStation("Bridgetown".to_string(), 27.0),
            WeatherStation("Brisbane".to_string(), 21.4),
            WeatherStation("Brussels".to_string(), 10.5),
            WeatherStation("Bucharest".to_string(), 10.8),
            WeatherStation("Budapest".to_string(), 11.3),
            WeatherStation("Bujumbura".to_string(), 23.8),
            WeatherStation("Bulawayo".to_string(), 18.9),
            WeatherStation("Burnie".to_string(), 13.1),
            WeatherStation("Busan".to_string(), 15.0),
            WeatherStation("Cabo San Lucas".to_string(), 23.9),
            WeatherStation("Cairns".to_string(), 25.0),
            WeatherStation("Cairo".to_string(), 21.4),
            WeatherStation("Calgary".to_string(), 4.4),
            WeatherStation("Canberra".to_string(), 13.1),
            WeatherStation("Cape Town".to_string(), 16.2),
            WeatherStation("Changsha".to_string(), 17.4),
            WeatherStation("Charlotte".to_string(), 16.1),
            WeatherStation("Chiang Mai".to_string(), 25.8),
            WeatherStation("Chicago".to_string(), 9.8),
            WeatherStation("Chihuahua".to_string(), 18.6),
            WeatherStation("Chișinău".to_string(), 10.2),
            WeatherStation("Chittagong".to_string(), 25.9),
            WeatherStation("Chongqing".to_string(), 18.6),
            WeatherStation("Christchurch".to_string(), 12.2),
            WeatherStation("City of San Marino".to_string(), 11.8),
            WeatherStation("Colombo".to_string(), 27.4),
            WeatherStation("Columbus".to_string(), 11.7),
            WeatherStation("Conakry".to_string(), 26.4),
            WeatherStation("Copenhagen".to_string(), 9.1),
            WeatherStation("Cotonou".to_string(), 27.2),
            WeatherStation("Cracow".to_string(), 9.3),
            WeatherStation("Da Lat".to_string(), 17.9),
            WeatherStation("Da Nang".to_string(), 25.8),
            WeatherStation("Dakar".to_string(), 24.0),
            WeatherStation("Dallas".to_string(), 19.0),
            WeatherStation("Damascus".to_string(), 17.0),
            WeatherStation("Dampier".to_string(), 26.4),
            WeatherStation("Dar es Salaam".to_string(), 25.8),
            WeatherStation("Darwin".to_string(), 27.6),
            WeatherStation("Denpasar".to_string(), 23.7),
            WeatherStation("Denver".to_string(), 10.4),
            WeatherStation("Detroit".to_string(), 10.0),
            WeatherStation("Dhaka".to_string(), 25.9),
            WeatherStation("Dikson".to_string(), -11.1),
            WeatherStation("Dili".to_string(), 26.6),
            WeatherStation("Djibouti".to_string(), 29.9),
            WeatherStation("Dodoma".to_string(), 22.7),
            WeatherStation("Dolisie".to_string(), 24.0),
            WeatherStation("Douala".to_string(), 26.7),
            WeatherStation("Dubai".to_string(), 26.9),
            WeatherStation("Dublin".to_string(), 9.8),
            WeatherStation("Dunedin".to_string(), 11.1),
            WeatherStation("Durban".to_string(), 20.6),
            WeatherStation("Dushanbe".to_string(), 14.7),
            WeatherStation("Edinburgh".to_string(), 9.3),
            WeatherStation("Edmonton".to_string(), 4.2),
            WeatherStation("El Paso".to_string(), 18.1),
            WeatherStation("Entebbe".to_string(), 21.0),
            WeatherStation("Erbil".to_string(), 19.5),
            WeatherStation("Erzurum".to_string(), 5.1),
            WeatherStation("Fairbanks".to_string(), -2.3),
            WeatherStation("Fianarantsoa".to_string(), 17.9),
            WeatherStation("Flores,  Petén".to_string(), 26.4),
            WeatherStation("Frankfurt".to_string(), 10.6),
            WeatherStation("Fresno".to_string(), 17.9),
            WeatherStation("Fukuoka".to_string(), 17.0),
            WeatherStation("Gabès".to_string(), 19.5),
            WeatherStation("Gaborone".to_string(), 21.0),
            WeatherStation("Gagnoa".to_string(), 26.0),
            WeatherStation("Gangtok".to_string(), 15.2),
            WeatherStation("Garissa".to_string(), 29.3),
            WeatherStation("Garoua".to_string(), 28.3),
            WeatherStation("George Town".to_string(), 27.9),
            WeatherStation("Ghanzi".to_string(), 21.4),
            WeatherStation("Gjoa Haven".to_string(), -14.4),
            WeatherStation("Guadalajara".to_string(), 20.9),
            WeatherStation("Guangzhou".to_string(), 22.4),
            WeatherStation("Guatemala City".to_string(), 20.4),
            WeatherStation("Halifax".to_string(), 7.5),
            WeatherStation("Hamburg".to_string(), 9.7),
            WeatherStation("Hamilton".to_string(), 13.8),
            WeatherStation("Hanga Roa".to_string(), 20.5),
            WeatherStation("Hanoi".to_string(), 23.6),
            WeatherStation("Harare".to_string(), 18.4),
            WeatherStation("Harbin".to_string(), 5.0),
            WeatherStation("Hargeisa".to_string(), 21.7),
            WeatherStation("Hat Yai".to_string(), 27.0),
            WeatherStation("Havana".to_string(), 25.2),
            WeatherStation("Helsinki".to_string(), 5.9),
            WeatherStation("Heraklion".to_string(), 18.9),
            WeatherStation("Hiroshima".to_string(), 16.3),
            WeatherStation("Ho Chi Minh City".to_string(), 27.4),
            WeatherStation("Hobart".to_string(), 12.7),
            WeatherStation("Hong Kong".to_string(), 23.3),
            WeatherStation("Honiara".to_string(), 26.5),
            WeatherStation("Honolulu".to_string(), 25.4),
            WeatherStation("Houston".to_string(), 20.8),
            WeatherStation("Ifrane".to_string(), 11.4),
            WeatherStation("Indianapolis".to_string(), 11.8),
            WeatherStation("Iqaluit".to_string(), -9.3),
            WeatherStation("Irkutsk".to_string(), 1.0),
            WeatherStation("Istanbul".to_string(), 13.9),
            WeatherStation("İzmir".to_string(), 17.9),
            WeatherStation("Jacksonville".to_string(), 20.3),
            WeatherStation("Jakarta".to_string(), 26.7),
            WeatherStation("Jayapura".to_string(), 27.0),
            WeatherStation("Jerusalem".to_string(), 18.3),
            WeatherStation("Johannesburg".to_string(), 15.5),
            WeatherStation("Jos".to_string(), 22.8),
            WeatherStation("Juba".to_string(), 27.8),
            WeatherStation("Kabul".to_string(), 12.1),
            WeatherStation("Kampala".to_string(), 20.0),
            WeatherStation("Kandi".to_string(), 27.7),
            WeatherStation("Kankan".to_string(), 26.5),
            WeatherStation("Kano".to_string(), 26.4),
            WeatherStation("Kansas City".to_string(), 12.5),
            WeatherStation("Karachi".to_string(), 26.0),
            WeatherStation("Karonga".to_string(), 24.4),
            WeatherStation("Kathmandu".to_string(), 18.3),
            WeatherStation("Khartoum".to_string(), 29.9),
            WeatherStation("Kingston".to_string(), 27.4),
            WeatherStation("Kinshasa".to_string(), 25.3),
            WeatherStation("Kolkata".to_string(), 26.7),
            WeatherStation("Kuala Lumpur".to_string(), 27.3),
            WeatherStation("Kumasi".to_string(), 26.0),
            WeatherStation("Kunming".to_string(), 15.7),
            WeatherStation("Kuopio".to_string(), 3.4),
            WeatherStation("Kuwait City".to_string(), 25.7),
            WeatherStation("Kyiv".to_string(), 8.4),
            WeatherStation("Kyoto".to_string(), 15.8),
            WeatherStation("La Ceiba".to_string(), 26.2),
            WeatherStation("La Paz".to_string(), 23.7),
            WeatherStation("Lagos".to_string(), 26.8),
            WeatherStation("Lahore".to_string(), 24.3),
            WeatherStation("Lake Havasu City".to_string(), 23.7),
            WeatherStation("Lake Tekapo".to_string(), 8.7),
            WeatherStation("Las Palmas de Gran Canaria".to_string(), 21.2),
            WeatherStation("Las Vegas".to_string(), 20.3),
            WeatherStation("Launceston".to_string(), 13.1),
            WeatherStation("Lhasa".to_string(), 7.6),
            WeatherStation("Libreville".to_string(), 25.9),
            WeatherStation("Lisbon".to_string(), 17.5),
            WeatherStation("Livingstone".to_string(), 21.8),
            WeatherStation("Ljubljana".to_string(), 10.9),
            WeatherStation("Lodwar".to_string(), 29.3),
            WeatherStation("Lomé".to_string(), 26.9),
            WeatherStation("London".to_string(), 11.3),
            WeatherStation("Los Angeles".to_string(), 18.6),
            WeatherStation("Louisville".to_string(), 13.9),
            WeatherStation("Luanda".to_string(), 25.8),
            WeatherStation("Lubumbashi".to_string(), 20.8),
            WeatherStation("Lusaka".to_string(), 19.9),
            WeatherStation("Luxembourg City".to_string(), 9.3),
            WeatherStation("Lviv".to_string(), 7.8),
            WeatherStation("Lyon".to_string(), 12.5),
            WeatherStation("Madrid".to_string(), 15.0),
            WeatherStation("Mahajanga".to_string(), 26.3),
            WeatherStation("Makassar".to_string(), 26.7),
            WeatherStation("Makurdi".to_string(), 26.0),
            WeatherStation("Malabo".to_string(), 26.3),
            WeatherStation("Malé".to_string(), 28.0),
            WeatherStation("Managua".to_string(), 27.3),
            WeatherStation("Manama".to_string(), 26.5),
            WeatherStation("Mandalay".to_string(), 28.0),
            WeatherStation("Mango".to_string(), 28.1),
            WeatherStation("Manila".to_string(), 28.4),
            WeatherStation("Maputo".to_string(), 22.8),
            WeatherStation("Marrakesh".to_string(), 19.6),
            WeatherStation("Marseille".to_string(), 15.8),
            WeatherStation("Maun".to_string(), 22.4),
            WeatherStation("Medan".to_string(), 26.5),
            WeatherStation("Mek'ele".to_string(), 22.7),
            WeatherStation("Melbourne".to_string(), 15.1),
            WeatherStation("Memphis".to_string(), 17.2),
            WeatherStation("Mexicali".to_string(), 23.1),
            WeatherStation("Mexico City".to_string(), 17.5),
            WeatherStation("Miami".to_string(), 24.9),
            WeatherStation("Milan".to_string(), 13.0),
            WeatherStation("Milwaukee".to_string(), 8.9),
            WeatherStation("Minneapolis".to_string(), 7.8),
            WeatherStation("Minsk".to_string(), 6.7),
            WeatherStation("Mogadishu".to_string(), 27.1),
            WeatherStation("Mombasa".to_string(), 26.3),
            WeatherStation("Monaco".to_string(), 16.4),
            WeatherStation("Moncton".to_string(), 6.1),
            WeatherStation("Monterrey".to_string(), 22.3),
            WeatherStation("Montreal".to_string(), 6.8),
            WeatherStation("Moscow".to_string(), 5.8),
            WeatherStation("Mumbai".to_string(), 27.1),
            WeatherStation("Murmansk".to_string(), 0.6),
            WeatherStation("Muscat".to_string(), 28.0),
            WeatherStation("Mzuzu".to_string(), 17.7),
            WeatherStation("N'Djamena".to_string(), 28.3),
            WeatherStation("Naha".to_string(), 23.1),
            WeatherStation("Nairobi".to_string(), 17.8),
            WeatherStation("Nakhon Ratchasima".to_string(), 27.3),
            WeatherStation("Napier".to_string(), 14.6),
            WeatherStation("Napoli".to_string(), 15.9),
            WeatherStation("Nashville".to_string(), 15.4),
            WeatherStation("Nassau".to_string(), 24.6),
            WeatherStation("Ndola".to_string(), 20.3),
            WeatherStation("New Delhi".to_string(), 25.0),
            WeatherStation("New Orleans".to_string(), 20.7),
            WeatherStation("New York City".to_string(), 12.9),
            WeatherStation("Ngaoundéré".to_string(), 22.0),
            WeatherStation("Niamey".to_string(), 29.3),
            WeatherStation("Nicosia".to_string(), 19.7),
            WeatherStation("Niigata".to_string(), 13.9),
            WeatherStation("Nouadhibou".to_string(), 21.3),
            WeatherStation("Nouakchott".to_string(), 25.7),
            WeatherStation("Novosibirsk".to_string(), 1.7),
            WeatherStation("Nuuk".to_string(), -1.4),
            WeatherStation("Odesa".to_string(), 10.7),
            WeatherStation("Odienné".to_string(), 26.0),
            WeatherStation("Oklahoma City".to_string(), 15.9),
            WeatherStation("Omaha".to_string(), 10.6),
            WeatherStation("Oranjestad".to_string(), 28.1),
            WeatherStation("Oslo".to_string(), 5.7),
            WeatherStation("Ottawa".to_string(), 6.6),
            WeatherStation("Ouagadougou".to_string(), 28.3),
            WeatherStation("Ouahigouya".to_string(), 28.6),
            WeatherStation("Ouarzazate".to_string(), 18.9),
            WeatherStation("Oulu".to_string(), 2.7),
            WeatherStation("Palembang".to_string(), 27.3),
            WeatherStation("Palermo".to_string(), 18.5),
            WeatherStation("Palm Springs".to_string(), 24.5),
            WeatherStation("Palmerston North".to_string(), 13.2),
            WeatherStation("Panama City".to_string(), 28.0),
            WeatherStation("Parakou".to_string(), 26.8),
            WeatherStation("Paris".to_string(), 12.3),
            WeatherStation("Perth".to_string(), 18.7),
            WeatherStation("Petropavlovsk-Kamchatsky".to_string(), 1.9),
            WeatherStation("Philadelphia".to_string(), 13.2),
            WeatherStation("Phnom Penh".to_string(), 28.3),
            WeatherStation("Phoenix".to_string(), 23.9),
            WeatherStation("Pittsburgh".to_string(), 10.8),
            WeatherStation("Podgorica".to_string(), 15.3),
            WeatherStation("Pointe-Noire".to_string(), 26.1),
            WeatherStation("Pontianak".to_string(), 27.7),
            WeatherStation("Port Moresby".to_string(), 26.9),
            WeatherStation("Port Sudan".to_string(), 28.4),
            WeatherStation("Port Vila".to_string(), 24.3),
            WeatherStation("Port-Gentil".to_string(), 26.0),
            WeatherStation("Portland (OR)".to_string(), 12.4),
            WeatherStation("Porto".to_string(), 15.7),
            WeatherStation("Prague".to_string(), 8.4),
            WeatherStation("Praia".to_string(), 24.4),
            WeatherStation("Pretoria".to_string(), 18.2),
            WeatherStation("Pyongyang".to_string(), 10.8),
            WeatherStation("Rabat".to_string(), 17.2),
            WeatherStation("Rangpur".to_string(), 24.4),
            WeatherStation("Reggane".to_string(), 28.3),
            WeatherStation("Reykjavík".to_string(), 4.3),
            WeatherStation("Riga".to_string(), 6.2),
            WeatherStation("Riyadh".to_string(), 26.0),
            WeatherStation("Rome".to_string(), 15.2),
            WeatherStation("Roseau".to_string(), 26.2),
            WeatherStation("Rostov-on-Don".to_string(), 9.9),
            WeatherStation("Sacramento".to_string(), 16.3),
            WeatherStation("Saint Petersburg".to_string(), 5.8),
            WeatherStation("Saint-Pierre".to_string(), 5.7),
            WeatherStation("Salt Lake City".to_string(), 11.6),
            WeatherStation("San Antonio".to_string(), 20.8),
            WeatherStation("San Diego".to_string(), 17.8),
            WeatherStation("San Francisco".to_string(), 14.6),
            WeatherStation("San Jose".to_string(), 16.4),
            WeatherStation("San José".to_string(), 22.6),
            WeatherStation("San Juan".to_string(), 27.2),
            WeatherStation("San Salvador".to_string(), 23.1),
            WeatherStation("Sana'a".to_string(), 20.0),
            WeatherStation("Santo Domingo".to_string(), 25.9),
            WeatherStation("Sapporo".to_string(), 8.9),
            WeatherStation("Sarajevo".to_string(), 10.1),
            WeatherStation("Saskatoon".to_string(), 3.3),
            WeatherStation("Seattle".to_string(), 11.3),
            WeatherStation("Ségou".to_string(), 28.0),
            WeatherStation("Seoul".to_string(), 12.5),
            WeatherStation("Seville".to_string(), 19.2),
            WeatherStation("Shanghai".to_string(), 16.7),
            WeatherStation("Singapore".to_string(), 27.0),
            WeatherStation("Skopje".to_string(), 12.4),
            WeatherStation("Sochi".to_string(), 14.2),
            WeatherStation("Sofia".to_string(), 10.6),
            WeatherStation("Sokoto".to_string(), 28.0),
            WeatherStation("Split".to_string(), 16.1),
            WeatherStation("St. John's".to_string(), 5.0),
            WeatherStation("St. Louis".to_string(), 13.9),
            WeatherStation("Stockholm".to_string(), 6.6),
            WeatherStation("Surabaya".to_string(), 27.1),
            WeatherStation("Suva".to_string(), 25.6),
            WeatherStation("Suwałki".to_string(), 7.2),
            WeatherStation("Sydney".to_string(), 17.7),
            WeatherStation("Tabora".to_string(), 23.0),
            WeatherStation("Tabriz".to_string(), 12.6),
            WeatherStation("Taipei".to_string(), 23.0),
            WeatherStation("Tallinn".to_string(), 6.4),
            WeatherStation("Tamale".to_string(), 27.9),
            WeatherStation("Tamanrasset".to_string(), 21.7),
            WeatherStation("Tampa".to_string(), 22.9),
            WeatherStation("Tashkent".to_string(), 14.8),
            WeatherStation("Tauranga".to_string(), 14.8),
            WeatherStation("Tbilisi".to_string(), 12.9),
            WeatherStation("Tegucigalpa".to_string(), 21.7),
            WeatherStation("Tehran".to_string(), 17.0),
            WeatherStation("Tel Aviv".to_string(), 20.0),
            WeatherStation("Thessaloniki".to_string(), 16.0),
            WeatherStation("Thiès".to_string(), 24.0),
            WeatherStation("Tijuana".to_string(), 17.8),
            WeatherStation("Timbuktu".to_string(), 28.0),
            WeatherStation("Tirana".to_string(), 15.2),
            WeatherStation("Toamasina".to_string(), 23.4),
            WeatherStation("Tokyo".to_string(), 15.4),
            WeatherStation("Toliara".to_string(), 24.1),
            WeatherStation("Toluca".to_string(), 12.4),
            WeatherStation("Toronto".to_string(), 9.4),
            WeatherStation("Tripoli".to_string(), 20.0),
            WeatherStation("Tromsø".to_string(), 2.9),
            WeatherStation("Tucson".to_string(), 20.9),
            WeatherStation("Tunis".to_string(), 18.4),
            WeatherStation("Ulaanbaatar".to_string(), -0.4),
            WeatherStation("Upington".to_string(), 20.4),
            WeatherStation("Ürümqi".to_string(), 7.4),
            WeatherStation("Vaduz".to_string(), 10.1),
            WeatherStation("Valencia".to_string(), 18.3),
            WeatherStation("Valletta".to_string(), 18.8),
            WeatherStation("Vancouver".to_string(), 10.4),
            WeatherStation("Veracruz".to_string(), 25.4),
            WeatherStation("Vienna".to_string(), 10.4),
            WeatherStation("Vientiane".to_string(), 25.9),
            WeatherStation("Villahermosa".to_string(), 27.1),
            WeatherStation("Vilnius".to_string(), 6.0),
            WeatherStation("Virginia Beach".to_string(), 15.8),
            WeatherStation("Vladivostok".to_string(), 4.9),
            WeatherStation("Warsaw".to_string(), 8.5),
            WeatherStation("Washington, D.C.".to_string(), 14.6),
            WeatherStation("Wau".to_string(), 27.8),
            WeatherStation("Wellington".to_string(), 12.9),
            WeatherStation("Whitehorse".to_string(), -0.1),
            WeatherStation("Wichita".to_string(), 13.9),
            WeatherStation("Willemstad".to_string(), 28.0),
            WeatherStation("Winnipeg".to_string(), 3.0),
            WeatherStation("Wrocław".to_string(), 9.6),
            WeatherStation("Xi'an".to_string(), 14.1),
            WeatherStation("Yakutsk".to_string(), -8.8),
            WeatherStation("Yangon".to_string(), 27.5),
            WeatherStation("Yaoundé".to_string(), 23.8),
            WeatherStation("Yellowknife".to_string(), -4.3),
            WeatherStation("Yerevan".to_string(), 12.4),
            WeatherStation("Yinchuan".to_string(), 9.0),
            WeatherStation("Zagreb".to_string(), 10.7),
            WeatherStation("Zanzibar City".to_string(), 26.0),
            WeatherStation("Zürich".to_string(), 9.3),
        ]
    }
}
