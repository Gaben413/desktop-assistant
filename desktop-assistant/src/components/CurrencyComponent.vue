<template>
        <div id="rate-exchanger">
            
            <div id="left-exchange" class="re-slots">
                <h3>Slot 1</h3>
                <select name="" id="currency1" v-model="currency_slot_1" @change="rate_exchange">
                    <option v-for="currency in currencies" :value="currency.value">{{ currency.name }}</option>
                </select>
                <input type="number" step=".01" class="currency-input" v-model="slot1_value" @change="rate_exchange">
            </div>

            <div id="mid-exchange" class="re-slots">
                <h3>=></h3>
            </div>

            <div id="right-exchange" class="re-slots">
                <h3>Slot 2</h3>
                <select name="" id="currency2" v-model="currency_slot_2" @change="rate_exchange">
                    <option v-for="currency in currencies" :value="currency.value">{{ currency.name }}</option>
                </select>
                <label>{{ exchanged_value }}</label>
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
    #rate-exchanger-container{
        margin: auto;
        width: 75%;
        padding: 10px;
    }
    #rate-exchanger{
        border-style: dashed;
        display: flex;
        width: auto;
    }
    .re-slots{
        text-align: center;
        padding: 10px;

        border-radius: 25px;
        width: 35%;

    }
    #left-exchange{
        
        background-color: red;
        
    }
    #mid-exchange{

        width: 5%;
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
    #exchange-button{
        background-color: green;
    }
</style>