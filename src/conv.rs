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

pub enum LUnit {
  Centimeter,
  Inches,
  Kilometer,
  Miles
}

impl fmt::Display for LUnit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let unit_name = match self {
      LUnit::Centimeter => "cm",
      LUnit::Inches     => "inch",
      LUnit::Kilometer  => "km",
      LUnit::Miles      => "miles"
    };

    write!(f, "[panjang] {}", unit_name)
  }
}

impl LUnit {
  fn as_unit(&self) -> String {
    match self {
      LUnit::Centimeter => String::from("cm"),
      LUnit::Inches     => String::from("inch"),
      LUnit::Kilometer  => String::from("km"),
      LUnit::Miles      => String::from("miles")
    }
  }
}

impl FromStr for LUnit  {
  type Err = String;

    fn from_str(unit: &str) -> Result<Self, Self::Err> {
      match unit {
        "cm"    => Ok(LUnit::Centimeter),
        "inch"  => Ok(LUnit::Inches),
        "km"    => Ok(LUnit::Kilometer),
        "miles" => Ok(LUnit::Miles),
        _ => Err(format!("Unit {} tidak didukung.", unit))
      }
    }
}

pub enum Unit {
  Temperature(TUnit),
  Length(LUnit)
}

impl Unit {
  fn as_unit(&self) -> String {
    let types = match self {
      Unit::Temperature(u) => u.as_unit(),
      Unit::Length(l) => l.as_unit()
    };
    types
  }
}

pub fn validate_arguments(from: Option<String>, to: Option<String>, value: Option<String>) 
  -> Result<(Unit, Unit, f64), String> {
  
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

  let from_unit = parse_unit(&from_unwrap)?;
  let to_unit = parse_unit(&to_unwrap)?;

  Ok((from_unit, to_unit, parsed_value))
}

fn parse_unit(unit_str: &String) -> Result<Unit, String> {
  let tunit = TUnit::from_str(unit_str);
  if let Ok(tunit) = tunit {
    return Ok(Unit::Temperature(tunit));
  }

  let lunit = LUnit::from_str(unit_str);
  if let Ok(lunit) = lunit {
    return Ok(Unit::Length(lunit));
  }

  Err(format!("Unit {} tidak didukung", unit_str))
}

fn other_to_celsius(from_unit: &TUnit, value: &f64) -> f64 {
  let celsius = match from_unit {
    TUnit::Celsius    => value.clone(),
    TUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
    TUnit::Kelvin     => value - 273.1
  };

  celsius
}

fn celsius_to_other(to_unit: &TUnit, celsius: &f64) -> f64 {
  let target_value = match to_unit {
    TUnit::Celsius    => celsius.clone(),
    TUnit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
    TUnit::Kelvin     => celsius + 273.15
  };

  target_value
}

fn convert_temperature(from_unit: &TUnit, to_unit: &TUnit, value: &f64) -> ConvHist {
  let celsius = other_to_celsius(from_unit, value);
  let to_value = celsius_to_other(to_unit, &celsius);

  let conv_hist = ConvHist {
    from_unit: from_unit.as_unit(),
    from_value: value.clone(),
    to_unit: to_unit.as_unit(),
    to_value: to_value
  };

  conv_hist
}

fn other_to_cm(from_unit: &LUnit, value: &f64) -> f64 {
  let cm = match from_unit {
    LUnit::Centimeter => value.clone(),
    LUnit::Inches     => value * 2.54,
    LUnit::Kilometer  => value * 100_000.0,
    LUnit::Miles      => value * 160_934.4,
  };

  cm
}

fn cm_to_other(to_unit: &LUnit, cm: &f64) -> f64 {
  let target_value = match to_unit {
    LUnit::Centimeter => cm.clone(),
    LUnit::Inches     => cm / 2.54,
    LUnit::Kilometer  => cm / 100_000.0,
    LUnit::Miles      => cm / 160_934.4,
  };

  target_value
}

fn convert_length(from_unit: &LUnit, to_unit: &LUnit, value: &f64) -> ConvHist {
  let cm = other_to_cm(from_unit, value);
  let to_value = cm_to_other(to_unit, &cm);

  let conv_hist = ConvHist {
    from_unit: from_unit.as_unit(),
    from_value: value.clone(),
    to_unit: to_unit.as_unit(),
    to_value: to_value
  };

  conv_hist
}

pub fn list_unit() {
  let units = vec![
    TUnit::Celsius.to_string(),
    TUnit::Fahrenheit.to_string(),
    TUnit::Kelvin.to_string(),
    LUnit::Centimeter.to_string(),
    LUnit::Inches.to_string(),
    LUnit::Kilometer.to_string(),
    LUnit::Miles.to_string()
  ];

  println!("Satuan yang didukung:");
  for (index, unit) in units.iter().enumerate()  {
    println!("{}. {}", (index + 1), unit);
  }
}

pub fn add_conversion(from_unit: &Unit, to_unit: &Unit, value: &f64) -> Result<bool, String> {
  let conv_hist = match (from_unit, to_unit) {
    (Unit::Temperature(from_tunit), Unit::Temperature(to_tunit)) => {
      Some(convert_temperature(from_tunit, to_tunit, value))
    },
    (Unit::Length(from_lunit), Unit::Length(to_lunit)) => {
      Some(convert_length(from_lunit, to_lunit, value))
    }
    _ => None
  };

  if let None = conv_hist {
    return Err(format!("Konversi unit {} ke unit {} tidak didukung", from_unit.as_unit(), to_unit.as_unit()));
  }

  let mut histories = load_hist()?;
  let conv_hist_unwrap = conv_hist.unwrap();

  println!("{}", conv_hist_unwrap);

  histories.push(conv_hist_unwrap);

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