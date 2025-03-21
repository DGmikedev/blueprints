use crate::mkr_fke_data;

    pub fn get_fake_data(codigo: &str, ini: &str,  fin: &str)-> String{

        let inicio: usize = ini.parse::<usize>().expect("Ha ocurrido un error fn[make_insert]");
        let finf: usize = fin.parse::<usize>().expect("Ha ocurrido un error fn[make_insert]");

        match codigo {

            "NAM" =>  return mkr_fke_data::get_name(),
            "NMF" => {
                        let name = format!("{} {} {}",
                        mkr_fke_data::get_name(),
                        mkr_fke_data::get_mdl_lst_name(),
                        mkr_fke_data::get_mdl_lst_name() 
                        );
                        return name
            },
            "APP" => return mkr_fke_data::get_mdl_lst_name(),
            "ADD" => return mkr_fke_data::get_addres(),
            "MAI" => return mkr_fke_data::get_email(),
            "PHO" => return mkr_fke_data::get_movil(),
            "CON" => return mkr_fke_data::get_countries(),
            "CNT" => return mkr_fke_data::get_continents(),
            "SLF" => return mkr_fke_data::get_num_dec(4,2),
            "SLI" => return mkr_fke_data::get_int_usize(2),
            "TIN" => return mkr_fke_data::get_int_usize(1),
            "SMA" => return mkr_fke_data::get_int_usize(2),
            "MED" => return mkr_fke_data::get_int_usize(3),
            "INT" => return mkr_fke_data::get_int_usize(4),
            "BIG" => return mkr_fke_data::get_int_usize(5),
            "FLO" => return mkr_fke_data::get_double_or_float(0),
            "DOU" => return mkr_fke_data::get_double_or_float(1),
            "DEC" => return mkr_fke_data::get_num_dec( inicio as usize, finf as usize ),
            "NUM" => return mkr_fke_data::get_num_dec( inicio as usize, finf as usize ),
            "CHA" => return mkr_fke_data::get_varchar_txt(inicio),
            "VAR" => return mkr_fke_data::get_varchar_txt(inicio),
            "TEX" => return mkr_fke_data::get_varchar_txt(inicio),
            "DAT" => return mkr_fke_data::get_full_date(3),
            "TIM" => return mkr_fke_data::get_full_date(1),
            "DTM" => return mkr_fke_data::get_full_date(0),
            "TMS" => return mkr_fke_data::get_full_date(3),
            "YEA" => return mkr_fke_data::get_full_date(2),
            "BIN" => return mkr_fke_data::get_binary(inicio),
            "VRB" => return mkr_fke_data::get_binary(inicio),
            "BOO" => return mkr_fke_data::get_boolean(),
            "JSO" => return mkr_fke_data::get_json(),
            // "ENU" => return ini
            // "SET" => return mkr_fke_data::get_name(),
            _ =>  return " ".to_string()
        }
    }

    /*
            <option value="N/A">SELECT</option>
            <option disabled   >-- Fast data ----</option>
            <option value="NAM">NAME</option>
            <option value="NMF">NAME FULL</option>
            <option value="APP">MIDDLE-LAST NAME</option>
            <option value="ADD">ADDRESS</option>
            <option value="MAI">EMAIL</option>
            <option value="PHO">PHONE</option>
            <option value="CON">COUNTRY</option>
            <option value="CNT">CONTINENT</option>
            <option value="SLF">SELL FLOAT</option>
            <option value="SLI">SELL INT</option>
            <option disable>---------------------</option>
            <option value="TIN">TINYINT</option>
            <option value="SMA">SMALLINT</option>
            <option value="MED">MEDIUMINT</option>
            <option value="INT">INT</option>
            <option value="BIG">BIGINT</option>
            <option disable>---------------------</option>
            <option value="FLO">FLOAT</option>
            <option value="DOU">DOUBLE</option>
            <option value="DEC">DECIMAL (P, S)</option>
            <option value="NUM">NUMERIC (P, S)</option>
	        <option disabled   >-------------</option>
            <option value="CHA">CHAR (N)</option>
            <option value="VAR">VARCHAR (N)</option>
            <option value="TEX">TEXT</option>
            <option disabled>-------------</option>
            <option value="DAT">DATE</option>       
            <option value="TIM">TIME</option>       
            <option value="DTM">DATETIME</option>   
            <option value="TMS">TIMESTAMP</option>  
            <option value="YEA">YEAR</option>       
            <option disabled>-------------</option>
            <option value="BIN">BINARY (N)</option>
            <option value="VRB">VARBINARY (N)</option>
            <option disabled   >-------------</option>
            <option value="BOO">BOOLEAN</option>
            <option value="JSO">JSON</option>
            <option value="ENU">ENUM</option>
            <option value="SET">SET</option>

     */