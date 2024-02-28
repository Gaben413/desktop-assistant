<template>
    <div id="inner-data">
        <div id="ram-display">
            <h3>RAM</h3>
            <p>{{ ram_used_display }}</p>
            <p>{{ ram_total_display }}</p>
        </div>

        <div id="cpu-display">
            <h3>CPU</h3>
            <p>{{ cpu_count }}</p>
            <p>{{ cpu_average_usage }}</p>
        </div>

        <h3>Storage</h3>
        <div class="disk-display" v-for="disk in disk_data">
            <p>{{ disk['disk_name'] }} | {{ disk['disk_mount_point'] }}</p>
            <p v-if="disk['disk_is_removable']">Removable</p>
            <p> {{ format_byte(disk['disk_available_space']) }} / {{ format_byte(disk['disk_total_space']) }}</p>
            <div :style="{'height': '15px', 'width': '100%', 'background-color': 'red'}">
                <div :style="{'height': '15px', 'width': get_storage_percentage(disk['disk_available_space'],disk['disk_total_space']), 'background-color': 'green'}"></div>
            </div>
            <p class="percentage-p">{{ get_storage_percentage(disk['disk_available_space'],disk['disk_total_space']) }}</p>

        </div>

        <button @click="get_system_data">Reload</button>
    </div>

</template>

<script>
    import { invoke } from "@tauri-apps/api/tauri";

    export default{
        data(){
            return{
                greetMsg: "",
                name: "Kay",
                ram_used_display: "00GB",
                ram_total_display: "/00GB RAM",
                cpu_count: 0,
                cpu_average_usage: 0,
                disk_data: [],
                desk_percentage: ['25%']
            }
        },
        mounted(){

            console.log("System Data:")
            
            this.get_system_data()
        },
        methods:{
            async get_system_data(){
                let json = JSON.parse(await invoke("get_system_data"))
                console.log(json);
                console.log(json['used_memory']*1e-9);

                this.ram_used_display = `${(json['used_memory']*1e-9).toFixed(2)}GB`;
                this.ram_total_display = `/${(json['total_memory']*1e-9).toFixed(2)}GB`;

                this.cpu_count = `${json['cpu_count']} cores`;
                this.cpu_average_usage = `${(json['cpu_average_usage']).toFixed(2)}%`;

                this.disk_data = json['disk_data'];
                /*
                for (let i = 0; i < json['disk_data'].length; i++) {
                    console.log(this.format_byte(json['disk_data'][i]['disk_total_space']))
                }
                */

            },
            format_byte(input_bytes){
                //console.log(`INPUT: ${input_bytes} | C1: ${1/(1e-6)} | C2: ${1/(1e-9)}`);
                if(input_bytes < (1/1e-6)){
                    return `${input_bytes} BYTES`;
                }if(input_bytes >= (1/1e-6) && input_bytes < (1/1e-9)){
                    return `${(input_bytes * 1e-6).toFixed(2)} MB`;
                }else if(input_bytes >= (1/1e-9) && input_bytes < (1/1e-12)){
                    return `${(input_bytes * 1e-9).toFixed(2)} GB`;
                }else if(input_bytes >= (1/1e-12)){
                    return `${(input_bytes * 1e-12).toFixed(2)} TB`;
                }
            },
            get_storage_percentage(current_ammount, max_ammount){
                return `${((100*current_ammount)/max_ammount).toFixed(2)}%`;
            }
            /*
            async function greet() {
                // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                greetMsg.value = await invoke("greet", { name: name.value });
            }
            */
        }
    }
</script>

<style>
    #inner-data{
        background-color: white;

        margin-right: 15px;

        padding-left: 5px;
    }

    h3{
        margin: 5px 0px 0px 0px;
    }

    #ram-display > p{
        margin: 0;
    }

    #cpu-display > p{
        margin: 0;
    }

    .disk-display{
        margin-bottom: 15px;
    }
    .disk-display > p{
        margin: 0;
    }

    .percentage-p{
        display: block;

        height: 100%;
        width: 100%;

        margin: 0;
    }
</style>