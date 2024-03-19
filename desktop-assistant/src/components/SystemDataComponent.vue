<template>
    <div id="inner-data">
        <div id="ram-display">
            <h3>RAM</h3>
            <p>{{ ram_used_display }} / {{ ram_total_display }}</p>
            <PercentageBarComponent :current_ammount=ram_raw_used :total_ammount=ram_raw_total bar_color="blue" bar_bg_color="#3366ff" />
        </div>

        <div id="cpu-display">
            <h3>CPU</h3>
            <p>{{ cpu_count }}</p>
            <p>{{ cpu_average_usage }}</p>
            <PercentageBarComponent :current_ammount=cpu_percentage :total_ammount=100 bar_color="blue" bar_bg_color="#3366ff" />

        </div>

        <h3>Storage</h3>
        <div class="disk-display" v-for="disk in disk_data">
            <p>{{ disk['disk_name'] }} | {{ disk['disk_mount_point'] }}</p>
            <p v-if="disk['disk_is_removable']">Removable</p>

            <p class="storage-p"> {{ format_byte(disk['disk_available_space']) }} / {{ format_byte(disk['disk_total_space']) }}</p>
            <PercentageBarComponent :current_ammount=disk.disk_available_space :total_ammount=disk.disk_total_space bar_color="green" bar_bg_color="red" />

        </div>

        <!--
        <button @click="get_system_data">Reload</button>
        -->
    </div>

</template>

<script>
    import { invoke } from "@tauri-apps/api/tauri";

    import PercentageBarComponent from "./PercentageBarComponent.vue";

    export default{
        components: {
            PercentageBarComponent
        },
        data(){
            return{
                interval: null,
                greetMsg: "",
                name: "Kay",
                
                ram_raw_used: 0,
                ram_raw_total: 0,
                ram_used_display: "00GB",
                ram_total_display: "/00GB RAM",

                cpu_count: 0,
                cpu_average_usage: 0,
                cpu_percentage: 0,

                disk_data: [],
                desk_percentage: ['25%']
            }
        },
        mounted(){
            this.get_system_data()

            this.interval = setInterval(() => {
                console.log("Fetch System Data");
                this.get_system_data()

            }, 2500);
        },
        unmounted(){
            clearInterval(this.interval);
        },
        methods:{
            async get_system_data(){
                let json = JSON.parse(await invoke("get_system_data"))
                //console.log(json);
                //console.log(json['used_memory']*1e-9);

                this.ram_raw_used = json['used_memory'];
                this.ram_raw_total = json['total_memory'];

                this.ram_used_display = `${(json['used_memory']*1e-9).toFixed(2)}GB`;
                this.ram_total_display = `${(json['total_memory']*1e-9).toFixed(2)}GB`;

                this.cpu_count = `${json['cpu_count']} cores`;
                this.cpu_average_usage = `${(json['cpu_average_usage']).toFixed(2)}%`;
                this.cpu_percentage = json['cpu_average_usage'];

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
        padding-right: 5px;

    }

    h3{
        margin: 5px 0px 0px 0px;
    }

    #ram-display > p{
        margin: 0;
        font-size: 10px;
    }

    #cpu-display > p{
        margin: 0;
        font-size: 15px;
    }

    .disk-display{
        margin-bottom: 15px;
    }
    .disk-display > p{
        margin: 0;
    }

    .storage-p{
        font-size: 10px;
    }
</style>