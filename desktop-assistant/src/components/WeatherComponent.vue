<template>
    <div id="w-grid">
        <div id="season">
            <p class="season-p" v-if="season()==1">SPRING</p>
            <p class="season-p" v-else-if="season()==2">SUMMER</p>
            <p class="season-p" v-else-if="season()==3">AUTUMN</p>
            <p class="season-p" v-else-if="season()==4">WINTER</p>
        </div>
        <div id="moon">
            <p class="moon-p" v-if="moon_phase==1">LAST QUARTER</p>
            <p class="moon-p" v-else-if="moon_phase==2">NEW MOON</p>
            <p class="moon-p" v-else-if="moon_phase==3">FIRST QUARTER</p>
            <p class="moon-p" v-else-if="moon_phase==4">FULL MOON</p>
        </div>
        <div id="temperature">
            <p class="temperature-p">{{ temperature_data['temperature_c'] }}</p>
        </div>
    </div>
    
</template>

<script>
    import { fetch } from '@tauri-apps/api/http';

    export default {

        data(){
            return{
                //SEASONS
                season: function(){

                    let current_day = new Date().getDate()
                    let current_month = new Date().getMonth()+1
                    let current_year = new Date().getFullYear()
                    let month_days = new Date(current_year, current_month, 0).getDate()

                    let rotation = 1;

                    let spring_months = [[3, 4, 5], [9, 10, 11]]
                    let summer_months = [[6, 7, 8], [12, 1, 2]]
                    let autumn_months = [[9, 10, 11], [3, 4, 5]]
                    let winter_months = [[12, 1, 2], [6, 7, 8]]

                    if (spring_months[rotation].includes(current_month)){
                        return 1;
                    } else if (summer_months[rotation].includes(current_month)){
                        return 2;
                    } else if (autumn_months[rotation].includes(current_month)){
                        return 3;
                    } else if (winter_months[rotation].includes(current_month)){
                        return 4;
                    } 
                },
                moon_phase: 1,
                temperature_data: {}
            }
        },
        async created() {
            //MOON PHASES
            let moon_data = await this.get_data();

            let result = moon_data.find(data => data.day == new Date().getDate())

            //console.log("Let's go")
            //console.log(result)

            if(result['phase'] == 'Last Quarter'){
                this.moon_phase = 1
            }else if(result['phase'] == 'New Moon'){
                this.moon_phase = 2
            }else if(result['phase'] == 'First Quarter'){
                this.moon_phase = 3
            }else if(result['phase'] == 'Full Moon'){
                this.moon_phase = 4
            }

            //TEMPERATURE
            let geolocation = await this.get_geolocation();
            console.log(geolocation)

            this.temperature_data = await this.get_temperature_data(geolocation);
            console.log(this.temperature_data)

        },
        methods: {
            get_data: async() => {

                let current_month = new Date().getMonth()+1
                let current_year = new Date().getFullYear()
                let month_days = new Date(current_year, current_month, 0).getDate()

                const response = await fetch(`https://aa.usno.navy.mil/api/moon/phases/year?year=${current_year}`, {
                    method: 'GET',
                    timeout: 30,
                });

                let obj_array = response['data']['phasedata'].filter(o => o.month == current_month);

                //console.log(obj_array)

                let index_check = 0;

                let days_obj = [];

                for (let i = 0; i < month_days; i++) {

                    let today_data = obj_array.find(date => date.day == (i+1))

                    if(today_data != undefined && today_data['phase'] != obj_array[index_check]['phase']
                    || (obj_array[index_check]['phase'] == "New Moon" || obj_array[index_check]['phase'] == "Full Moon")){
                        index_check++;
                    }

                    if(index_check == obj_array.length) index_check = 0;

                    let phase = obj_array[index_check]['phase'];

                    days_obj.push({
                        day: (i+1),
                        phase: phase
                    })

                }

                //console.log(days_obj);

                return days_obj

            },

            get_geolocation: async () => {
                return new Promise(async (resolve, reject)=>{
                    navigator.geolocation.getCurrentPosition((position) => {
                        resolve({
                            latitude: position.coords.latitude,
                            longitude: position.coords.longitude
                        })

                    })
                })
                
            },
            get_temperature_data: async (geolocation) => {
                const response_raw = 
                    await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${geolocation.latitude}&longitude=${geolocation.longitude}&current=temperature_2m,relative_humidity_2m,is_day,precipitation,rain,cloud_cover`, {
                        method: 'GET',
                        timeout: 30,
                    });

                let response = {
                    temperature_c: response_raw['data']['current']['temperature_2m'] + "°C",
                    temperature_f: ((response_raw['data']['current']['temperature_2m']*(9/5))+32) + "°F",
                    humidity: response_raw['data']['current']['relative_humidity_2m'] + "%",
                    is_day: response_raw['data']['current']['is_day'] == 1,
                    cloud_cover: response_raw['data']['current']['cloud_cover'] + "%",
                    precipitation: response_raw['data']['current']['precipitation'] + "mm",
                    rain: response_raw['data']['current']['rain'] + "mm"
                }

                return response
            }
        }
    }
</script>

<style scoped>
    #w-grid{
        display: grid;

        grid-template-columns: auto auto auto;

        justify-content: center;

        justify-content: space-between;
    }
    .season-p{
        margin: 0;
    }
    .moon-p{
        margin: 0;
    }
    .temperature-p{
        margin: 0;
    }
</style>