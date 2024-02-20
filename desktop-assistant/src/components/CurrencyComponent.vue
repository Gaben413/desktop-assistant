<template>
        <div id="c-grid">
            <div id="left-exchange" class="c-grid-cell">
                <h3>Slot 1</h3>
                <select name="" id="currency1" v-model="currency_slot_1" @change="rate_exchange">
                    <option v-for="currency in currencies" :value="currency.value">{{ currency.name }}</option>
                </select>
                <input type="number" step=".01" class="currency-input" v-model="slot1_value" @change="rate_exchange">
            </div>

            <div id="mid-exchange" class="c-grid-cell">
                <p>=></p>
            </div>

            <div id="right-exchange" class=" c-grid-cell">
                <h3>Slot 2</h3>
                <select name="" id="currency2" v-model="currency_slot_2" @change="rate_exchange">
                    <option v-for="currency in currencies" :value="currency.value">{{ currency.name }}</option>
                </select>
                <h2>{{ exchanged_value }}</h2>
                <!--
                <input type="number" step=".01" class="currency-input">
                -->
            </div>

            <!--
            <button id="exchange-button" @click="rate_exchange">Change</button>
            -->
        </div>
</template>

<script>
    import rate_data from '../assets/rate_exchange.json';

    export default {
        data(){
            return{
                test_data: 5,
                currencies: [
                    {
                        name: "Dollar (USA)",
                        value: 1
                    }
                ],
                currency_slot_1: 1,
                currency_slot_2: 1,
                slot1_value: 1,
                exchanged_value: 1
            }
        },
        created(){
            for (let i = 0; i < rate_data['data'].length; i++) {
                this.currencies.push({
                    name: `${rate_data['data'][i]['currency']} (${rate_data['data'][i]['country']})`,
                    value: rate_data['data'][i]['exchange_rate']
                })
            }
        },
        methods: {
            test(){
                console.log("Test");
            },
            rate_exchange(){
                this.exchanged_value = this.money_exchange(this.currency_slot_1, this.currency_slot_2, this.slot1_value);
                //console.log(`${this.currency_slot_1} | ${this.currency_slot_2}`)
                console.log(this.exchanged_value);
                
            },
            money_exchange(slot1_exchange, slot2_exchange, slot1_value){
                return (slot1_value/slot1_exchange)*slot2_exchange;
            }
        }
    }
</script>

<style>
    #c-grid{
        display: grid;

        grid-template-columns: auto auto auto;

        justify-content: center;

        column-gap: 5px;
        row-gap: 5px;
    }
    .c-grid-cell{
        width: 200px;
        height: 150px;

        text-align: center;
        padding: 10px;

        border-radius: 25px;
    }
    .c-grid-cell > h2{
        margin-top: 10px;
    }

    #left-exchange{
        
        background-color: darkgray;
        
    }
    #mid-exchange{
        
        width: 50px;

        /* background-color: gray; */
        
    }
    #mid-exchange > p{
        
        width: 100%;

        padding: 125% 0px;
        margin: 0px;

        text-align: center;

        font-size: 25px;
    }
    #right-exchange{

        background-color: lightgray;
        
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
        text-align: center;
    }
    #exchange-button{
        background-color: green;
    }
</style>