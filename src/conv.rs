use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConvHist {
  from_unit: String,
  from_value: f64,
  to_unit: String,
  to_value: f64
}

impl fmt::Display for ConvHist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} = {} {}",self.from_value, self.from_unit, self.to_value, self.to_unit)
    }
}

pub enum TUnit {
  Celsius,
  Fahrenheit,
  Kelvin
}

impl fmt::Display for TUnit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let unit_name = match self {
        TUnit::Celsius    => "celsius",
        TUnit::Fahrenheit => "fahrenheit",
        TUnit::Kelvin     => "kelvin"
      };

      write!(f, "[suhu] {}", unit_name)
  }
}

impl TUnit {
  fn as_unit(&self) -> String {
      match self {
          TUnit::Celsius    => String::from("°C"),
          TUnit::Fahrenheit => String::from("°F"),
          TUnit::Kelvin     => String::from("K"),
      }
  }
}

impl FromStr for TUnit  {
  type Err = String;

    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit {
            "celsius"    => Ok(TUnit::Celsius),
            "fahrenheit" => Ok(TUnit::Fahrenheit),
            "kelvin"     => Ok(TUnit::Kelvin),
            _ => Err(format!("Unit {} tidak didukung.", unit))
        }
    }
}

pub fn validate_arguments(from: Option<String>, to: Option<String>, value: Option<String>) -> Result<(TUnit, TUnit, f64), String> {
  
  if let None = from {
    return Err("Nilai `from` wajib diisi.".to_string());
  }

  if let None = to {
    return Err("Nilai `to` wajib diisi.".to_string());
  }

  if let None = value {
    return Err("Nilai `value` wajib diisi.".to_string());
  }

  let parsed_value = match value.unwrap().to_string().parse::<f64>() {
      Ok(v) => v,
      Err(_) => return Err("Pastikan value adalah number".to_string())
  };

  let from_unwrap = from.unwrap();
  let to_unwrap = to.unwrap();

  let from_unit = TUnit::from_str(&from_unwrap)?;
  let to_unit = TUnit::from_str(&to_unwrap)?;

  return Ok((from_unit, to_unit, parsed_value));
}

fn fahrenheit_to_celsius(fahrenheit: &f64) -> f64 {
  (fahrenheit - 32.0) * 5.0 / 9.0
}

fn kelvin_to_celsius(kelvin: &f64) -> f64 {
  kelvin - 273.1
}

fn celsius_to_fahrenheit(celsius: &f64) -> f64 {
  celsius * 9.0 / 5.0 + 32.0
}

fn celsius_to_kelvin(celsius: &f64) -> f64 {
  celsius + 273.15
}

fn other_to_celsius(from_unit: &TUnit, value: &f64) -> f64 {
  let celsius = match from_unit {
      TUnit::Celsius => value.clone(),
      TUnit::Fahrenheit => fahrenheit_to_celsius(value),
      TUnit::Kelvin => kelvin_to_celsius(value)
  };

  celsius
}

fn celsius_to_other(to_unit: &TUnit, celsius: &f64) -> f64 {
  let target_value = match to_unit {
      TUnit::Celsius => celsius.clone(),
      TUnit::Fahrenheit => celsius_to_fahrenheit(celsius),
      TUnit::Kelvin => celsius_to_kelvin(celsius)
  };

  target_value
}

fn convert_temperature(from_unit: &TUnit, to_unit: &TUnit, value: &f64) -> f64 {
  let celsius = other_to_celsius(from_unit, value);
  let target_value = celsius_to_other(to_unit, &celsius);
  target_value
}

pub fn list_unit() {
  let units = vec![TUnit::Celsius, TUnit::Fahrenheit, TUnit::Kelvin];

  println!("Satuan yang didukung:");
  for (index, unit) in units.iter().enumerate()  {
      println!("{}. {}", (index + 1), unit);
  }
}

pub fn add_conversion(from_unit: TUnit, to_unit: TUnit, value: f64) -> Result<bool, String> {
  let target_value = convert_temperature(&from_unit, &to_unit, &value);
  let mut histories = load_hist()?;

  let conv_hist = ConvHist {
    from_unit: from_unit.as_unit(),
    to_unit: to_unit.as_unit(),
    from_value: value,
    to_value: target_value
  };

  println!("{}", conv_hist);

  histories.push(conv_hist);

  save_hist(&histories)?;

  Ok(true)
}

pub fn show_hist() -> Result<bool, String> {
  let histories = load_hist()?;

  println!("Riwayat Konversi:");
  if histories.is_empty() {
    println!("Belum ada riwayat konversi.");
  } else {
    for (index, history) in histories.iter().enumerate() {
      let number = index + 1;
      println!("{}. {}", number, history);
    }
  }

  Ok(true)
}

pub fn load_hist() -> Result<Vec<ConvHist>, String> {
  let path = Path::new("conversion.json");
  if !path.exists() {
    File::create("conversion.json").expect("Gagal membuat file");
    std::fs::write("conversion.json", "[]").expect("Gagal set default file content");
  }

  let hist_json: Result<String, std::io::Error> = std::fs::read_to_string("conversion.json");
  let hist_json: String = match hist_json {
      Ok(s) => s,
      Err(e) => return Err(e.to_string()),
  };

  let histories: Result<Vec<ConvHist>, serde_json::Error>= serde_json::from_str(&hist_json);
  let histories: Vec<ConvHist> = match histories {
      Ok(t) => t,
      Err(e) => return Err(e.to_string()),
  };
  
  Ok(histories)
}

pub fn save_hist(histories: &Vec<ConvHist>) -> Result<bool, String> {
  let hist_json = serde_json::to_string(histories);
  let hist_json = match hist_json {
    Ok(t) => t,
    Err(e) => return Err(e.to_string())
  };

  let write_to_file = std::fs::write("conversion.json", hist_json);
  match write_to_file {
    Ok(_) => return Ok(true),
    Err(e) => return Err(e.to_string())
  };
}

pub fn clear_hist() -> Result<bool, String> {
  let write_to_file = std::fs::write("conversion.json", "[]");
  match write_to_file {
    Ok(_) => return Ok(true),
    Err(e) => return Err(e.to_string())
  };
}