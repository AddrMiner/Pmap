# PMap

a port scanning tool that efficiently discovers the majority of open ports from all 65K ports in the whole network.

PMap uses the correlation of ports to build an open port correlation graph of each network, using a reinforcement learning framework to update the correlation graph based on feedback results and dynamically adjust the order of port scanning.

## Installation

1. [Install Rust - Rust Programming Language (rust-lang.org)](https://www.rust-lang.org/tools/install)

2. Install Pmap

    Windows:
    
    ```shell
    .\install_windows.ps1
    ```
    
    Linux:
    
    ```shell
    ./install_linux.sh
    ```
    
    Mac:
    
    ```shell
    ./install_macos.sh
    ```

## Usage

Example:

```shell
smap -m p4 -b 10m -t 42.81.179.50-42.81.179.180 -p 80,443,22,21,53 -a pmap_sampling_pro=0.1 -a pmap_budget=2 -a pmap_batch_num=2
```

```shell
smap -m p6 -b 10m -t 240e:928:1400:105::b@125-128 -p 80,443,22,21,53 -a pmap_sampling_pro=0.1 -a pmap_budget=2 -a pmap_batch_num=2
```

## Parameters

- pmap_budget: Scan budget, recommended number of ports allocated per address during the formal scanning stage.
- pmap_sampling_pro:  Pre-scan ratio.
- pmap_min_sample_num: Minimum sample size during pre-scan.
- pmap_batch_num: Recommended number of rounds for pmap scanning (For example, if the value is set to 10, all target addresses in the recommended scanning stage will be divided into 10 parts. After scanning one part completely (all recommended ports), the next part will be scanned. During this process, based on user settings, it will be determined whether to update the probability-related graphs or not.)
- pmap_allow_graph_iter: Whether to allow probability-related graph updates
- pmap_use_hash_recorder: Whether to use a hash table as the default recorder. If set to true, a hash table (suitable for scenarios with a large overall range and higher recommended rounds) will be used as the default recorder. If set to false, a bitmap (suitable for scenarios with a smaller overall range and higher activity ratio) will be used as the default recorder.

## Notes

The online probe version of pmap is a sub-mode of smap. The code in this repository is consistent with the code in the smap repository. For more detailed information and usage instructions, please refer to [AddrMiner/smap (github.com)](https://github.com/AddrMiner/smap).