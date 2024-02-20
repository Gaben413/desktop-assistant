<template>
    <WeatherComponent />
    <!--
    <h3>Grid Test</h3>

    <div class="grid-container">
        <div class="grid-cell cell-1"></div>
        <div class="grid-cell cell-2"></div>
        <div class="grid-cell cell-3"></div>
        <div class="grid-cell cell-4"></div>
        <div class="grid-cell cell-1"></div>
        <div class="grid-cell cell-2"></div>
        <div class="grid-cell cell-3"></div>
        <div class="grid-cell cell-4"></div>
    </div>
    -->
    <h2 style="text-align: center;">Clocks</h2>
    <div class="grid-container">
        <div v-for="clock_data in clock_data_array" class="grid-cell">
            <ClockComponent :time_zone_data=clock_data :key="clock_data.id" />
        </div>
    </div>

    <CurrencyComponent />

</template>

<script setup>
    import { onMounted, onUnmounted } from 'vue';
    import ClockComponent from '../components/ClockComponent.vue';
    import CurrencyComponent from '../components/CurrencyComponent.vue';
    import WeatherComponent from '../components/WeatherComponent.vue';

    import clock_data_json from '../assets/clock_data.json'

    let clock_data_array = clock_data_json['time_areas'];

    //const prop = defineProps(['foo'])
    let foo = {
        "title": "Lord",
        "name": "English"
    }

    let intervalLoop;

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

   function rate_exchange(){
    console.log("Exchange");
   }
</script>

<style>
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
</style>