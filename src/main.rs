// Copyright 2025 David Araújo
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{process::exit};

use clap::Parser;

use subprocess::Exec;

#[derive(Parser)]
#[command(
    author = "David Araújo",
    version = "v0.1.0",
    about = "Geo-location spoofer for Android devices.",
    after_help = "Example:\ngeospoofer --locale fr-CA --mcc 214 --mnc 21 --gps \"40.4168,-3.7038\""
)]
struct Args {
    #[arg(short, long, help = "Language-Country code (e.g., en-US, es-ES, ja-JP)", default_value = "en-US")]
    locale: String,

    #[arg(long, help = "Mobile Country Code (e.g., 310)", default_value = "310")]
    mcc: String,

    #[arg(long, help = "Mobile Network Code (e.g., 260)", default_value = "260")]
    mnc: String,

    #[arg(long, help = "GPS Latitude (e.g., 40.7580)", default_value = "40.7580")]
    latitude: String,

    #[arg(long, help = "GPS Longitude (e.g., -73.9855)", default_value = "-73.9855")]
    longitude: String,

    #[arg(long, help = "List all possible locale codes" ,action= clap::ArgAction::SetTrue)]
    list_locales: bool,
}

const _LOCALE_CODE: &[&str] = &["aa-ER","af-NA","af-ZA","am-ET","ar-EG","ar-DZ","ar-BH","ar-DJ","ar-ER","ar-IQ","ar-IL","ar-YE","ar-JO","ar-QA","ar-KM","ar-KW","ar-LB","ar-LY","ar-MA","ar-MR","ar-OM","ar-PS","ar-SA","ar-SO","ar-SD","ar-SY","ar-TD","ar-TN","ar-AE","ay-BO","az-AZ","be-BY","bn-BD","bi-VU","bs-BA","bs-ME","bg-BG","byn-ER","ca-AD","cs-CZ","ch-GU","ch-MP","da-DK","de-BE","de-DE","de-LI","de-LU","de-AT","de-CH","de-VA","dv-MV","dz-BT","el-GR","el-CY","en-AS","en-AI","en-AQ","en-AG","en-AU","en-BS","en-BB","en-BZ","en-BM","en-BW","en-IO","en-CK","en-CW","en-DM","en-ER","en-SZ","en-FK","en-FJ","en-FM","en-GM","en-GH","en-GI","en-GD","en-GU","en-GG","en-GY","en-HM","en-HK","en-IN","en-IM","en-IE","en-JM","en-JE","en-VG","en-VI","en-KY","en-CM","en-CA","en-KE","en-KI","en-UM","en-CC","en-LS","en-LR","en-MW","en-MT","en-MH","en-MU","en-MS","en-NA","en-NR","en-NZ","en-NG","en-NU","en-MP","en-NF","en-PK","en-PW","en-PG","en-PH","en-PN","en-PR","en-RW","en-MF","en-SB","en-ZM","en-WS","en-SC","en-SL","en-ZW","en-SG","en-SX","en-SH","en-KN","en-LC","en-VC","en-ZA","en-SD","en-GS","en-SS","en-TZ","en-TK","en-TO","en-TT","en-TC","en-TV","en-UG","en-VU","en-US","en-GB","en-CX","et-EE","fan-GQ","fo-FO","fa-IR","fj-FJ","fi-FI","fr-GQ","fr-BE","fr-BJ","fr-BF","fr-BI","fr-CD","fr-DJ","fr-CI","fr-FR","fr-GF","fr-PF","fr-TF","fr-MC","fr-GA","fr-GP","fr-GG","fr-GN","fr-HT","fr-JE","fr-CM","fr-CA","fr-KM","fr-LB","fr-LU","fr-MG","fr-ML","fr-MQ","fr-YT","fr-NC","fr-NE","fr-CG","fr-RE","fr-RW","fr-MF","fr-BL","fr-CH","fr-SN","fr-SC","fr-PM","fr-TG","fr-TD","fr-VU","fr-VA","fr-WF","fr-CF","ff-BF","ff-GN","ga-IE","gv-IM","gn-AR","gn-PY","ht-HT","he-IL","hif-FJ","hi-IN","hr-BA","hr-HR","hr-ME","hu-HU","hy-AM","hy-CY","id-ID","is-IS","it-IT","it-SM","it-CH","it-VA","ja-JP","kl-GL","ka-GE","kk-KZ","km-KH","rw-RW","ky-KG","kg-CD","ko-KP","ko-KR","kun-ER","ku-IQ","lo-LA","la-VA","lv-LV","ln-CD","ln-CG","lt-LT","lb-LU","lu-CD","mh-MH","mk-MK","mg-MG","mt-MT","mn-MN","mi-NZ","ms-BN","ms-SG","my-MM","na-NR","nr-ZA","nd-ZW","ne-NP","nl-AW","nl-BE","nl-CW","nl-BQ","nl-NL","nl-MF","nl-SX","nl-SR","nn-BV","nn-NO","nb-BV","nb-NO","no-BV","no-NO","no-SJ","nrb-ER","ny-MW","pa-AW","pa-CW","pl-PL","pt-AO","pt-GQ","pt-BR","pt-GW","pt-CV","pt-MO","pt-MZ","pt-TL","pt-PT","pt-ST","ps-AF","qu-BO","rar-CK","rm-CH","ro-MD","ro-RO","rtm-FJ","rn-BI","ru-AQ","ru-BY","ru-KZ","ru-KG","ru-RU","ru-TJ","ru-TM","ru-UZ","sg-CF","si-LK","sk-SK","sk-CZ","sl-SI","sm-AS","sm-WS","sn-ZW","so-SO","st-LS","st-ZA","es-GQ","es-AR","es-BZ","es-BO","es-CL","es-CR","es-DO","es-EC","es-SV","es-GU","es-GT","es-HN","es-CO","es-CU","es-MX","es-NI","es-PA","es-PY","es-PE","es-PR","es-ES","es-UY","es-VE","es-EH","sq-AL","sq-XK","sq-ME","sr-BA","sr-XK","sr-ME","sr-RS","ss-SZ","ss-ZA","ssy-ER","sw-CD","sw-KE","sw-TZ","sw-UG","sv-AX","sv-FI","sv-SE","ta-SG","ta-LK","tg-TJ","th-TH","tig-ER","ti-ER","to-TO","tn-BW","tn-ZA","ts-ZA","tk-AF","tk-TM","tr-TR","tr-CY","uk-UA","ur-PK","uz-AF","uz-UZ","ve-ZA","vi-VN","xh-ZA","zh-CN","zh-HK","zh-MO","zh-SG","zh-TW","ms-MY","zu-ZA"];

fn set_prop(prop: &str, value: &str) {
    let status = Exec::cmd("setprop")
        .arg(prop)
        .arg(value)
        .capture()
        .expect("Failed to execute adb command");
    if !status.success() {
        eprintln!("Failed to set property {} to {}", prop, value);
    }
}

fn get_prop(prop: &str) -> String {
    let status = Exec::cmd("getprop")
        .arg(prop)
        .capture()
        .expect("Failed to execute adb command");
    if !status.success() {
        eprintln!("Failed to get property {}", prop);
    } else {
        println!("{}: {}", prop, status.stdout_str().trim());
    }
    status.stdout_str()
}

fn main() {
    let args = Args::parse();

    let lang = args.locale.split('-').next().unwrap_or("en");

    let country = args
        .locale
        .split('-')
        .nth(1)
        .unwrap_or("US")
        .to_string()
        .to_lowercase();

    // Checking if locale code is valid
    let locale_code = format!("{}-{}", lang, country);
    if !_LOCALE_CODE.contains(&locale_code.as_str()) {
        eprintln!("Warning: The locale code '{}' is not valid.", locale_code);
        exit(1);
    }

    // List all possible locale codes
    if args.list_locales {
        println!("Possible locale codes:");
        for code in _LOCALE_CODE {
            println!("{}", code);
        }
        exit(0);
    }

    println!(
        "Setting locale to: {}-{}\nSetting MCC to: {}\nSetting MNC to: {}\nSetting GPS coordinates to: {},{}",
        lang, country, args.mcc, args.mnc, args.latitude, args.longitude
    );

    // Set locale settings
    set_prop("persist.sys.locale", &args.locale);
    set_prop("persist.sys.language", &lang);
    set_prop("persist.sys.country", &country);

    // Restart Zygote to apply locale changes
    set_prop("ctl.restart", "zygote");

    // Set telephony/SIM carrier data
    set_prop("gsm.sim.operator.iso-country", &country);
    set_prop("gsm.sim.operator.numeric", &format!("{}{}", args.mcc, args.mnc));
    set_prop("gsm.operator.iso-country", &country);
    set_prop("gsm.operator.numeric", &format!("{}{}", args.mcc, args.mnc));

    // Set GPS location
    let geo_status = Exec::cmd("geo")
        .arg("fix")
        .arg(&args.longitude)
        .arg(&args.latitude)
        .capture()
        .expect("Failed to execute adb geo command");
    if !geo_status.success() {
        eprintln!("Failed to set GPS location to {},{}", args.latitude, args.longitude);
    }

    // Display new settings
    println!("--- New Settings Applied ---");
    get_prop("persist.sys.locale");
    get_prop("persist.sys.language");
    get_prop("persist.sys.country");
    get_prop("gsm.sim.operator.iso-country");
    get_prop("gsm.sim.operator.numeric");

    println!("--- Spoofing Complete ---");
}
