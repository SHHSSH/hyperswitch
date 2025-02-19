import json

# Sample Input Data
data = {
      "DE": [
      {
        "value": "Baden-Württemberg",
        "code": "BW"
      },
      {
        "value": "Bavaria",
        "code": "BY"
      },
      {
        "value": "Berlin",
        "code": "BE"
      },
      {
        "value": "Brandenburg",
        "code": "BB"
      },
      {
        "value": "Bremen",
        "code": "HB"
      },
      {
        "value": "Hamburg",
        "code": "HH"
      },
      {
        "value": "Hessen",
        "code": "HE"
      },
      {
        "value": "Lower Saxony",
        "code": "NI"
      },
      {
        "value": "Mecklenburg-Vorpommern",
        "code": "MV"
      },
      {
        "value": "North Rhine-Westphalia",
        "code": "NW"
      },
      {
        "value": "Rhineland-Palatinate",
        "code": "RP"
      },
      {
        "value": "Saarland",
        "code": "SL"
      },
      {
        "value": "Saxony",
        "code": "SN"
      },
      {
        "value": "Saxony-Anhalt",
        "code": "ST"
      },
      {
        "value": "Schleswig-Holstein",
        "code": "SH"
      },
      {
        "value": "Thuringia",
        "code": "TH"
      }
    ]
}

def generate_rust_enum_and_match(data, country_code):
    states = data.get(country_code, [])
    
    if not states:
        print(f"No data found for country: {country_code}")
        return
    
    # Generate Rust Enum
    enum_variants = "\n    ".join(f"{s['code']}," for s in states)
    rust_enum = f"enum StateCode {{\n    {enum_variants}\n}}\n"
    
    # Generate Rust Match Statement
    match_arms = "\n        ".join(f'\"{s["value"]}\" => Some(StateCode::{s["code"]}),' for s in states)
    rust_match = f"""
impl StateCode {{
    fn from_name(name: &str) -> Option<Self> {{
        match name {{
            {match_arms}
            _ => None,
        }}
    }}
}}
"""
    
    return rust_enum + rust_match