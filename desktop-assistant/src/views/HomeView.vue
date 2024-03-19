<template>
    <div id="home-grid">
        <div id="side-bar">
            <SystemDataComponent />
        </div>

        <div :class="classObject">
            <h1>Settings</h1>
            <button id="toggle-button" @click="settings_toggle">Settings</button>
        </div>
        
        <div id="main-screen">
            <WeatherComponent />

            <TimerComponent />
            
            <div class="grid-container">
                <div v-for="clock_data in clock_data_array" class="grid-cell">
                    <ClockComponent :time_zone_data=clock_data :key="clock_data.id" />
                </div>
            </div>

            <CurrencyComponent />
        </div>
    </div>
    

</template>

<script setup>
    import { onMounted, onUnmounted, reactive } from 'vue';
    import ClockComponent from '../components/ClockComponent.vue';
    import CurrencyComponent from '../components/CurrencyComponent.vue';
    import WeatherComponent from '../components/WeatherComponent.vue';
    import TimerComponent from '../components/TimerComponent.vue';
    import SystemDataComponent from '../components/SystemDataComponent.vue';

    import clock_data_json from '../assets/clock_data.json'

    let clock_data_array = clock_data_json['time_areas'];

    //const prop = defineProps(['foo'])
    let foo = {
        "title": "Lord",
        "name": "English"
    }

    let intervalLoop;

    let classObject = reactive({
        settings: true,
        settings_on: false,
        settings_off: true
    })

    /*
    onMounted(() => {
        intervalLoop = setInterval(() => {
            console.log("Created loop");
        }, 1000);
    });

    onUnmounted(() => {
        clearInterval(intervalLoop);
    })
    */

    function settings_toggle(){
        classObject['settings_on'] = !classObject['settings_on'];
        classObject['settings_off'] = !classObject['settings_off'];
    }

   function rate_exchange(){
    console.log("Exchange");
   }
</script>

<style>
    #toggle-button{
        position: absolute;
        right: 10px;
        bottom: 15px;

        background-color: lightgray;
    }
    .grid-container{
        display: grid;

        grid-template-columns: auto auto auto auto;

        justify-content: center;

        column-gap: 5px;
        row-gap: 5px;
    }
    .grid-cell{
        width: 200px;
        height: 200px;
    }
    .cell-1{
        background-color: red;
    }
    .cell-2{
        background-color: blue;
    }
    .cell-3{
        background-color: green;
    }
    .cell-4{
        background-color: yellow;
    }

    #home-grid{
        display: grid;

        grid-template-columns: 125px auto;
    }

    .settings{
        position: absolute;

        background-color: white;

        width: 75%;
        height: 100%;

    }
    .settings_off{
        z-index: -1;
        left: -63%;

        transition: left 1s;
    }
    .settings_on{
        z-index: 5;
        left: 0%;

        transition: left 1s;
    }
</style>