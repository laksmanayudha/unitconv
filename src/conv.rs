use std::str::FromStr;
use std::fmt;

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

pub fn validate_arguments(from: Option<String>, to: Option<String>, value: Option<String>)
  -> Result<(TUnit, TUnit, f64), String> {
  
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

pub fn convert_temperature(from_unit: &TUnit, to_unit: &TUnit, value: &f64) {
  let celsius = other_to_celsius(from_unit, value);
  let target_value = celsius_to_other(to_unit, &celsius);
  print!("Hasil konversi: {}  {}", target_value, to_unit);
}

pub fn list_unit() {
  let units = vec![TUnit::Celsius, TUnit::Fahrenheit, TUnit::Kelvin];

  println!("Satuan yang didukung:");
  for (index, unit) in units.iter().enumerate()  {
      println!("{}. {}", (index + 1), unit);
  }
}