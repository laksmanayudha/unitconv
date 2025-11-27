use std::str::FromStr;
use std::fmt;

pub enum TemperatureUnit {
  Celsius,
  Fahrenheit,
  Kelvin
}

impl fmt::Display for TemperatureUnit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let unit_name = match self {
        TemperatureUnit::Celsius    => "celsius",
        TemperatureUnit::Fahrenheit => "fahrenheit",
        TemperatureUnit::Kelvin     => "kelvin"
      };

      write!(f, "[suhu] {}", unit_name)
  }
}

impl FromStr for TemperatureUnit  {
  type Err = String;

    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit {
            "celsius"    => Ok(TemperatureUnit::Celsius),
            "fahrenheit" => Ok(TemperatureUnit::Fahrenheit),
            "kelvin"     => Ok(TemperatureUnit::Kelvin),
            _ => Err(format!("Unit {} tidak didukung.", unit))
        }
    }
}

pub fn validate_arguments(from: Option<String>, to: Option<String>, value: Option<String>)
  -> Result<(TemperatureUnit, TemperatureUnit, f64), String> {
  
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

  let from_unit = TemperatureUnit::from_str(&from_unwrap)?;
  let to_unit = TemperatureUnit::from_str(&to_unwrap)?;

  return Ok((from_unit, to_unit, parsed_value));
}

pub fn convert_temperature(from_unit: TemperatureUnit, to_unit: TemperatureUnit, value: f64) -> Result<f64, String> {
  Ok(0.0)
}