<template>
    <div class="clock-container">
        <h3>Clock {{ this.time_zone_data.city_name }}</h3>
        <h3>({{ this.time_zone_data.tz_name }})</h3>
        <p>{{ this.display_time }}</p>
        <p>{{ this.time_zone_data.tz_code }}</p>
    </div>
    
</template>

<!--
<script setup>
    import { onBeforeMount, onBeforeUnmount } from 'vue';

    const prop = defineProps(['foo'])

    let display_time = "Nothing";

    let intervalLoop;

    onBeforeMount(() => {
        intervalLoop = setInterval(() => {
            console.log("Fetch Time Loop");
            
            let new_display_time = new Date().toLocaleTimeString("pt-BR", {timeZone: "America/Sao_Paulo"}).toString()
            display_time = new_display_time;
            console.log(display_time);
        }, 1000);
    });

    onBeforeUnmount(() => {
        clearInterval(intervalLoop);
    })

</script>
-->

<script>
    export default{
        props:['time_zone_data'],
        data(){
            return {
                display_time: "00:00:00",
                intervalLoop: null
            }
        },
        beforeCreate(){
            this.intervalLoop = setInterval(() => {
                console.log("Fetch Time Loop");
                //console.log(this.time_zone_data)
                
                this.display_time = new Date().toLocaleTimeString("pt-BR", {timeZone: this.time_zone_data.tz_code}).toString()
        }, 1000);
        },
        unmounted(){
            clearInterval(this.intervalLoop);
        }
    }
</script>

<style>
    .clock-container{
        border-style: dashed;
        border-radius: 15px;
        
        margin: 5px;
        padding: 10px;

        text-align: center;
    }
</style>