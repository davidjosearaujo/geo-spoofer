use std::{process::exit, error::Error};

use clap::Parser;
use subprocess::Exec;
use reqwest;
use serde_json::Value;

// --- Data Structures ---

#[derive(Debug)]
struct Country {
    country_code: String, 
    language_code: String,
    name: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

// --- Clap Argument Parsing ---

#[derive(Parser)]
#[command(
    author = "David Araújo",
    version = "v0.1.0",
    about = "Geo-location spoofer for Android devices.",
    after_help = "Example:\ngeospoofer --locale fr-CA --mcc 214 --mnc 21"
)]
struct Args {
    #[arg(short, long, help = "Language-Country code (e.g., en-US, es-ES, ja-JP)", default_value = "en-US")]
    locale: String,

    #[arg(long, help = "Mobile Country Code (e.g., 310)", default_value = "310")]
    mcc: String,

    #[arg(long, help = "Mobile Network Code (e.g., 260)", default_value = "260")]
    mnc: String,

    #[arg(long, help = "List all possible locale codes" ,action= clap::ArgAction::SetTrue)]
    list_locales: bool,
}

// --- Locale Code Data ---

const LOCALE_CODES: &[&str] = &["aa-ER", "af-NA", "af-ZA", "am-ET", "ar-EG", "ar-DZ", "ar-BH", "ar-DJ", "ar-ER", "ar-IQ", "ar-IL", "ar-YE", "ar-JO", "ar-QA", "ar-KM", "ar-KW", "ar-LB", "ar-LY", "ar-MA", "ar-MR", "ar-OM", "ar-PS", "ar-SA", "ar-SO", "ar-SD", "ar-SY", "ar-TD", "ar-TN", "ar-AE", "ay-BO", "az-AZ", "be-BY", "bn-BD", "bi-VU", "bs-BA", "bs-ME", "bg-BG", "byn-ER", "ca-AD", "cs-CZ", "ch-GU", "ch-MP", "da-DK", "de-BE", "de-DE", "de-LI", "de-LU", "de-AT", "de-CH", "de-VA", "dv-MV", "dz-BT", "el-GR", "el-CY", "en-AS", "en-AI", "en-AQ", "en-AG", "en-AU", "en-BS", "en-BB", "en-BZ", "en-BM", "en-BW", "en-IO", "en-CK", "en-CW", "en-DM", "en-ER", "en-SZ", "en-FK", "en-FJ", "en-FM", "en-GM", "en-GH", "en-GI", "en-GD", "en-GU", "en-GG", "en-GY", "en-HM", "en-HK", "en-IN", "en-IM", "en-IE", "en-JM", "en-JE", "en-VG", "en-VI", "en-KY", "en-CM", "en-CA", "en-KE", "en-KI", "en-UM", "en-CC", "en-LS", "en-LR", "en-MW", "en-MT", "en-MH", "en-MU", "en-MS", "en-NA", "en-NR", "en-NZ", "en-NG", "en-NU", "en-MP", "en-NF", "en-PK", "en-PW", "en-PG", "en-PH", "en-PN", "en-PR", "en-RW", "en-MF", "en-SB", "en-ZM", "en-WS", "en-SC", "en-SL", "en-ZW", "en-SG", "en-SX", "en-SH", "en-KN", "en-LC", "en-VC", "en-ZA", "en-SD", "en-GS", "en-SS", "en-TZ", "en-TK", "en-TO", "en-TT", "en-TC", "en-TV", "en-UG", "en-VU", "en-US", "en-GB", "en-CX", "et-EE", "fan-GQ", "fo-FO", "fa-IR", "fj-FJ", "fi-FI", "fr-GQ", "fr-BE", "fr-BJ", "fr-BF", "fr-BI", "fr-CD", "fr-DJ", "fr-CI", "fr-FR", "fr-GF", "fr-PF", "fr-TF", "fr-MC", "fr-GA", "fr-GP", "fr-GG", "fr-GN", "fr-HT", "fr-JE", "fr-CM", "fr-CA", "fr-KM", "fr-LB", "fr-LU", "fr-MG", "fr-ML", "fr-MQ", "fr-YT", "fr-NC", "fr-NE", "fr-CG", "fr-RE", "fr-RW", "fr-MF", "fr-BL", "fr-CH", "fr-SN", "fr-SC", "fr-PM", "fr-TG", "fr-TD", "fr-VU", "fr-VA", "fr-WF", "fr-CF", "ff-BF", "ff-GN", "ga-IE", "gv-IM", "gn-AR", "gn-PY", "ht-HT", "he-IL", "hif-FJ", "hi-IN", "hr-BA", "hr-HR", "hr-ME", "hu-HU", "hy-AM", "hy-CY", "id-ID", "is-IS", "it-IT", "it-SM", "it-CH", "it-VA", "ja-JP", "kl-GL", "ka-GE", "kk-KZ", "km-KH", "rw-RW", "ky-KG", "kg-CD", "ko-KP", "ko-KR", "kun-ER", "ku-IQ", "lo-LA", "la-VA", "lv-LV", "ln-CD", "ln-CG", "lt-LT", "lb-LU", "lu-CD", "mh-MH", "mk-MK", "mg-MG", "mt-MT", "mn-MN", "mi-NZ", "ms-BN", "ms-SG", "my-MM", "na-NR", "nr-ZA", "nd-ZW", "ne-NP", "nl-AW", "nl-BE", "nl-CW", "nl-BQ", "nl-NL", "nl-MF", "nl-SX", "nl-SR", "nn-BV", "nn-NO", "nb-BV", "nb-NO", "no-BV", "no-NO", "no-SJ", "nrb-ER", "ny-MW", "pa-AW", "pa-CW", "pl-PL", "pt-AO", "pt-GQ", "pt-BR", "pt-GW", "pt-CV", "pt-MO", "pt-MZ", "pt-TL", "pt-PT", "pt-ST", "ps-AF", "qu-BO", "rar-CK", "rm-CH", "ro-MD", "ro-RO", "rtm-FJ", "rn-BI", "ru-AQ", "ru-BY", "ru-KZ", "ru-KG", "ru-RU", "ru-TJ", "ru-TM", "ru-UZ", "sg-CF", "si-LK", "sk-SK", "sk-CZ", "sl-SI", "sm-AS", "sm-WS", "sn-ZW", "so-SO", "st-LS", "st-ZA", "es-GQ", "es-AR", "es-BZ", "es-BO", "es-CL", "es-CR", "es-DO", "es-EC", "es-SV", "es-GU", "es-GT", "es-HN", "es-CO", "es-CU", "es-MX", "es-NI", "es-PA", "es-PY", "es-PE", "es-PR", "es-ES", "es-UY", "es-VE", "es-EH", "sq-AL", "sq-XK", "sq-ME", "sr-BA", "sr-XK", "sr-ME", "sr-RS", "ss-SZ", "ss-ZA", "ssy-ER", "sw-CD", "sw-KE", "sw-TZ", "sw-UG", "sv-AX", "sv-FI", "sv-SE", "ta-SG", "ta-LK", "tg-TJ", "th-TH", "tig-ER", "ti-ER", "to-TO", "tn-BW", "tn-ZA", "ts-ZA", "tk-AF", "tk-TM", "tr-TR", "tr-CY", "uk-UA", "ur-PK", "uz-AF", "uz-UZ", "ve-ZA", "vi-VN", "xh-ZA", "zh-CN", "zh-HK", "zh-MO", "zh-SG", "zh-TW", "ms-MY", "zu-ZA"];

// --- Core Logic Functions ---

fn get_country_by_locale(mut country: Country, locale: &str) -> Result<Country, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get("https://cdn.simplelocalize.io/public/v1/locales").send()?;
    let resp_body = resp.error_for_status()?.text()?;

    let locales_value: Value = serde_json::from_str(&resp_body)?;
    
    let locales_array = locales_value
        .as_array()
        .ok_or_else(|| "API response is not a JSON array".to_string())?;

    if let Some(locale_info) = locales_array.iter().find(|info| info["locale"] == locale) {
        let country_data = &locale_info["country"];
        
        if let (Some(name_str), Some(lat), Some(lon)) = (
            country_data["name"].as_str(),
            country_data["capital_latitude"].as_f64(),
            country_data["capital_longitude"].as_f64(),
        ) {
            country.name = Some(name_str.to_string());
            country.latitude = Some(lat);
            country.longitude = Some(lon);
            return Ok(country);
        } else {
            return Err(format!("Locale '{}' found but missing country name or capital coordinates in API response.", locale).into());
        }
    } else {
        return Err(format!("Locale '{}' not found in remote API response.", locale).into());
    }
}

fn set_prop(prop: &str, value: &str) {
    let status = Exec::cmd("adb")
        .arg("shell")
        .arg("setprop")
        .arg(prop)
        .arg(value)
        .capture();
        
    if let Ok(s) = status {
        if !s.success() {
            eprintln!("Failed to set property **{}** to **{}**. Error: {}", prop, value, s.stderr_str());
        }
    } else if let Err(e) = status {
        eprintln!("Failed to execute adb setprop command: {}", e);
    }
}

fn get_prop(prop: &str) {
    let status = Exec::cmd("adb")
        .arg("shell")
        .arg("getprop")
        .arg(prop)
        .capture();
        
    if let Ok(s) = status {
        if !s.success() {
            eprintln!("Failed to get property **{}**. Error: {}", prop, s.stderr_str());
        } else {
            println!("{}: {}", prop, s.stdout_str().trim());
        }
    } else if let Err(e) = status {
        eprintln!("Failed to execute adb getprop command: {}", e);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.list_locales {
        println!("Possible locale codes:");
        for code in LOCALE_CODES {
            println!("{}", code);
        }
        exit(0);
    }

    if !LOCALE_CODES.contains(&args.locale.as_str()) {
        eprintln!("Error: The locale code '**{}**' is not valid or supported.", args.locale);
        exit(1);
    }

    // Split locale (e.g., "en-US") into language ("en") and country ("US")
    let mut parts = args.locale.split('-');
    let lang = parts.next().unwrap_or("en").to_string();
    let country_code = parts.next().unwrap_or("US").to_lowercase(); 

    let initial_country = Country {
        country_code: country_code, 
        language_code: lang,
        name: None,
        latitude: None,
        longitude: None,
    };

    let country = get_country_by_locale(initial_country, &args.locale)?;
    
    let latitude = country.latitude.ok_or("Could not retrieve capital latitude for locale.")?;
    let longitude = country.longitude.ok_or("Could not retrieve capital longitude for locale.")?;

    // 0. Ensuring adb is running as root
    let _ = Exec::cmd("adb")
        .arg("wait-for-device")
        .arg("root")
        .capture();

    println!("Applying settings for locale: **{}** ({})", args.locale, country.name.as_deref().unwrap_or("N/A"));
    println!("Spoofing GPS to: {}, {}", latitude, longitude);

    // 1. Set locale properties
    set_prop("persist.sys.locale", &args.locale);
    set_prop("persist.sys.language", &country.language_code);
    set_prop("persist.sys.country", &country.country_code);

    // 2. Restart Zygote to apply locale changes
    set_prop("ctl.restart", "zygote");

    // 3. Set telephony/SIM carrier data
    let gsm_numeric = format!("{}{}", args.mcc, args.mnc);
    set_prop("gsm.sim.operator.iso-country", &country.country_code);
    set_prop("gsm.sim.operator.numeric", &gsm_numeric);
    set_prop("gsm.operator.iso-country", &country.country_code);
    set_prop("gsm.operator.numeric", &gsm_numeric);

    // 4. Set GPS location using ADB `geo fix` command
    let geo_status = Exec::cmd("adb")
        .arg("emu")
        .arg("geo")
        .arg("fix")
        .arg(latitude.to_string())
        .arg(longitude.to_string())
        .capture();
        
    match geo_status {
        Ok(s) => {
            if !s.success() {
                eprintln!("Failed to set GPS location (adb shell geo fix)");
            } else {
                println!("**GPS location set successfully.**");
            }
        },
        Err(e) => {
            eprintln!("Failed to execute adb geo command: {}", e);
        }
    }

    // Display new settings
    println!("\n--- New Settings Applied (Check Device) ---");
    get_prop("persist.sys.locale");
    get_prop("gsm.sim.operator.numeric");
    get_prop("gsm.operator.iso-country");
    
    Ok(())
}