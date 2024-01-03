use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stations = [
        "Hamburg", "Bulawayo", "Palembang", "St. John's", "Cracow", "Bridgetown", 
        "Istanbul", "Roseau", "Conakry", "Nuuk", "Kabul", "Tirana", "Algiers", 
        "Pago Pago", "Andorra la Vella", "Luanda", "The Valley", "Saint John's", 
        "Buenos Aires", "Yerevan", "Oranjestad", "Canberra", "Vienna", "Baku", 
        "Nassau", "Manama", "Dhaka", "Bridgetown", "Minsk", "Brussels", "Belmopan", 
        "Porto-Novo", "Hamilton", "Thimphu", "Sucre", "Kralendijk", "Sarajevo", 
        "Gaborone", "Brasilia", "Diego Garcia", "Bandar Seri Begawan", "Sofia", 
        "Ouagadougou", "Bujumbura", "Praia", "Phnom Penh", "Douala", "Ottawa", 
        "Praia", "Bangui", "N'Djamena", "Santiago", "Beijing", "Flying Fish Cove", 
        "West Island", "Bogota", "Moroni", "Avarua", "Zagreb", "Havana", "Willemstad", 
        "Nicosia", "Prague", "Copenhagen", "Djibouti", "Roseau", "Santo Domingo",
        "Quito", "Cairo", "San Salvador", "Malabo", "Asmara", "Tallinn", "Addis Ababa", 
        "Stanley", "Torshavn", "Suva", "Helsinki", "Paris", "Cayenne", "Papeete", 
        "Libreville", "Banjul", "Tbilisi", "Berlin", "Accra", "Gibraltar", "Athens", 
        "Nuuk", "St. George's", "Basse-Terre", "Hagatna", "Guatemala City", "St. Peter Port", 
        "Conakry", "Bissau", "Georgetown", "Port-au-Prince", "Tegucigalpa", "Budapest", 
        "Reykjavik", "New Delhi", "Jakarta", "Tehran", "Baghdad", "Dublin", "Douglas", 
        "Jerusalem", "Rome", "Kingston", "Tokyo", "Saint Helier", "Amman", "Astana", 
        "Nairobi", "Tarawa Atoll", "Pyongyang", "Seoul", "Pristina", "Kuwait City", 
        "Bishkek", "Vientiane", "Riga", "Beirut", "Maseru", "Monrovia", "Tripoli", 
        "Vaduz", "Vilnius", "Luxembourg", "Skopje", "Antananarivo", "Lilongwe", "Kuala Lumpur", 
        "Male", "Bamako", "Valletta", "Majuro", "Fort-de-France", "Nouakchott", "Port Louis", 
        "Mamoudzou", "Mexico City", "Chisinau", "Monaco", "Ulaanbaatar", "Podgorica", 
        "Plymouth", "Rabat", "Maputo", "Naypyidaw", "Windhoek", "Yaren", "Kathmandu", 
        "Amsterdam", "Noumea", "Wellington", "Managua", "Niamey", "Abuja", "Alofi", 
        "Kingston", "Saipan", "Oslo", "Muscat", "Islamabad", "Melekeok", "Panama City", 
        "Port Moresby", "Asuncion", "Lima", "Manila", "Adamstown", "Warsaw", "Lisbon", 
        "San Juan", "Doha", "Brazzaville", "Bucharest", "Moscow", "Kigali", "Gustavia", 
        "Jamestown", "Basseterre", "Castries", "Marigot", "Saint-Pierre", "Kingstown", 
        "Apia", "San Marino", "Sao Tome", "Riyadh", "Dakar", "Belgrade", "Victoria", 
        "Freetown", "Singapore", "Philipsburg", "Bratislava", "Ljubljana", "Honiara", 
        "Mogadishu", "Pretoria", "Grytviken", "Juba", "Madrid", "Colombo", "Khartoum", 
        "Paramaribo", "Longyearbyen", "Mbabane", "Stockholm", "Bern", "Damascus", "Taipei", 
        "Dushanbe", "Dar es Salaam", "Bangkok", "Lome", "Nuku'alofa", "Port of Spain", 
        "Tunis", "Ankara", "Ashgabat", "Grand Turk", "Funafuti", "Kampala", "Kiev", 
        "Abu Dhabi", "London", "Washington, D.C.", "Montevideo", "Tashkent", "Port Vila", 
        "Vatican City", "Caracas", "Hanoi", "Mata-Utu", "El Aaiun", "Sana'a", "Lusaka", "Harare"
    ];
    
    let file = File::create("/Users/chuck/scratch/onebrc/resources/measurements.csv")?;
    let mut writer = BufWriter::new(file);
    let mut rng = rand::thread_rng();
    let station_dist = Uniform::from(0..stations.len());
    let temp_dist = Uniform::from(-30.0..45.0);

    for _ in 0..1_000_000_000 {
        let station = stations[station_dist.sample(&mut rng)];
        let temp = temp_dist.sample(&mut rng);
        writeln!(writer, "{};{:.1}", station, temp)?;
    }

    Ok(())
}
