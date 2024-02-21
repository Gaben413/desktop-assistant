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
            TEMPERATURE
        </div>
    </div>
    
</template>



<script setup>
    import { fetch } from '@tauri-apps/api/http';

    let now_date = new Date();
    let current_day = now_date.getDate();
    let current_month = now_date.getMonth()+1;
    let current_year = now_date.getFullYear();
    let month_days = new Date(current_year, current_month, 0).getDate();

    console.log(current_year)

    //SEASON
    
    //[Meteorological Seasons, Oppositive Seasons]
    let spring_months = [[3, 4, 5], [9, 10, 11]];
    let summer_months = [[6, 7, 8], [12, 1, 2]];
    let autumn_months = [[9, 10, 11], [3, 4, 5]];
    let winter_months = [[12, 1, 2], [6, 7, 8]];

    let season = function(){
        let rotation = 1;

        if (spring_months[rotation].includes(current_month)){
            return 1;
        } else if (summer_months[rotation].includes(current_month)){
            return 2;
        } else if (autumn_months[rotation].includes(current_month)){
            return 3;
        } else if (winter_months[rotation].includes(current_month)){
            return 4;
        } 
    };

    //console.log(season())

    //MOON

    async function get_data(){
        return new Promise(async (resolve, reject)=>{
            const response = await fetch('https://aa.usno.navy.mil/api/moon/phases/year?year=2024', {
                method: 'GET',
                timeout: 30,
            });

            let obj_array = response['data']['phasedata'].filter(o => o.month == current_month)

            console.log(obj_array)

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

            console.log(days_obj);

            resolve(days_obj)
        })
        
    }

    let moon_phase = 1;

    (async () => {
        const moon_data = await get_data();
        let today_data = moon_data.find(data => data.day == current_day);

        if(today_data.phase == 'First Quarter'){
            moon_phase = 3;
        }

        console.log(moon_phase)
    })();


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
</style>