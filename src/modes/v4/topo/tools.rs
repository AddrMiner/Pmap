use std::net::Ipv4Addr;
use std::process::exit;
use std::sync::Arc;
use log::error;
use rand::prelude::StdRng;
use rand::Rng;
use crate::core::conf::modules_config::ModuleConf;
use crate::core::conf::tools::args_parse::ip::ipv4_pattern::{parse_ipv4_pattern, parse_pattern_local_range_v4};
use crate::modes::v4::Topo4;
use crate::modules::probe_modules::topology_probe::topo_mod_v4::TopoModV4;
use crate::SYS;

impl Topo4 {
    pub fn topo4_get_target_ips(tar_ips_str: &String, rand_bits: String, rng: &mut StdRng) -> (u32, u32, u32, Vec<(u32, u32)>, u32) {
        
        if tar_ips_str.contains("@") {
            let (ip_bits_num, mut base_ip_val, mask, parts, mut max_ip) = parse_ipv4_pattern(tar_ips_str);

            if rand_bits != "0" {
                // 如果 rand_bits 为 0, 表示不存在指定的随机位
                // 注意: 在模式位中的随机位无效

                // 生成随机ip值
                let rand_ip_val: u32 = rng.gen();

                // 生成真正的随机值
                let rand_mask = Self::get_rand_mask(rand_bits, mask);
                let rand_val = rand_ip_val & rand_mask;

                // 获取随机处理后的 base_ip 和 max_ip
                base_ip_val = (base_ip_val & (!rand_mask)) | rand_val;
                max_ip = (max_ip & (!rand_mask)) | rand_val;
            }

            (ip_bits_num, base_ip_val, mask, parts, max_ip)
        } else {
            // 如果未给定目标ip
            // 生成随机ip值
            let rand_ip_val: u32 = rng.gen();
            
            let rand_ip = Ipv4Addr::from(rand_ip_val);
            parse_ipv4_pattern(&String::from(format!("{}@{}", rand_ip, tar_ips_str)))
        }
    }

    /// 获取随机位掩码， 注意：随机位为1， 非随机位为0
    fn get_rand_mask(rand_bits: String, mask: u32) -> u32 {
        let s: Vec<&str> = rand_bits.split(',').collect();

        let mut rand_mask: u32 = u32::MAX;
        let mut pre_last: u32 = 0;
        for ps in s {   // 每个片段

            let (first, last) = parse_pattern_local_range_v4(ps);

            if pre_last >= first {
                // 当前片段的首索引 必须 大于上一个片段的 尾索引
                error!("{} {}", SYS.get_info("err", "ipv4_pattern_local_part_err"), ps);
                exit(1)
            } else {
                pre_last = last;
            }

            let part_len = last - first + 1;
            let left_move_len = 32 - last;

            //cur_mask:[  1..                         |   0.. (片段大小)  |  1..          ]

            //  mask1: [  1.. (32 - 片段大小 - 偏移量)  |  0.. (片段大小)  |  0.. (偏移量)   ]
            //  mask2: [  0.. (32 - 片段大小 - 偏移量)  |  0..            |  1.. (偏移量)   ]
            //         [  (32 - 偏移量) 或 片段最后一个元素的索引([1,32])   |    (偏移量)     ]

            let mut mask1;
            let mask1_left_move = part_len + left_move_len;
            if mask1_left_move == 32 {
                mask1 = 0;
            } else {
                mask1 = u32::MAX;
                mask1 = mask1 << mask1_left_move;
            }

            let mut mask2;
            if last == 32 {
                mask2 = 0;
            } else {
                mask2 = u32::MAX;
                mask2 = (mask2 << last) >> last;
            }


            let cur_mask = mask1 | mask2;

            rand_mask = rand_mask & cur_mask;
        }

        (!rand_mask) & mask
    }
    
    
    pub fn get_sub_probe(para:&str, mod_conf:ModuleConf) -> Option<Arc<TopoModV4>> {
        
        match mod_conf.get_info(&para.to_string()) {
            None => None,
            Some(name) => {
                Some(Arc::new(TopoModV4::new(&name, mod_conf)))
            }
        }
        
    }
}