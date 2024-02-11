<template>
    <h1>Home View</h1>
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

    <div id="rate-exchanger">
        <div id="left-exchange" class="re-slots">
            <h3>Currency 1</h3>
            <select name="" id="currency1">
                <option value="USD">USD</option>
                <option value="BRL">BRL</option>
                <option value="EUR">EUR</option>
            </select>
            <label>USD: </label>
            <input type="number" step=".01" class="currency-input">
        </div>

        <div id="mid-exchange" class="re-slots">
            <h3>=></h3>
        </div>

        <div id="right-exchange" class="re-slots">
            <h3>Currency 2</h3>
            <select name="" id="currency2">
                <option value="USD">USD</option>
                <option value="BRL">BRL</option>
                <option value="EUR">EUR</option>
            </select>
            <label>USD: </label>
            <input type="number" step=".01" class="currency-input">
        </div>

        <button @click="rate_exchange">Change</button>

    </div>

</template>

<script setup>
    import { onMounted, onUnmounted } from 'vue';
    import ClockComponent from '../components/ClockComponent.vue';

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
    #rate-exchanger{
        border-style: dashed;
        display: flex;
    }
    .re-slots{
        text-align: center;
        padding: 10px;

        border-radius: 25px;
    }
    #left-exchange{
        
        background-color: red;
        
    }
    #mid-exchange{
        
        padding-top: 5%;
        
        font-size: 25px;
        background-color: gray;
        
    }
    #right-exchange{
        background-color: yellow;
        
    }
    #currency1, #currency2{
        width: 100%;
        height: 25px;

        border-style: none;
        border-radius: 15px;

        margin-bottom: 15px;
    }
    .currency-input{
        width: 100px;
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
</style>