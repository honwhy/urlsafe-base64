<script>
    import {encode, decode} from 'urlsafe-base64'
    let orig = '';
    let result = '';
    let type = 'encode/decode'
	function handleEncode(event) {
        if(!orig) {
            return false
        }
        type = 'encode'
		result = encode(orig)
	}
    function handleDecode(event) {
        if(!orig) {
            return false
        }
        type = 'decode'
        try {
            result = decode(orig)
        } catch(e) {
            result = 'decode error'
        }
		
	}
</script>

<h1>urlsafe-base64 demo</h1>
<div class="compo">
    <div class="compo-item">
        <textarea bind:value={orig}/>
        <div>{type}:</div>
        <textarea disabled bind:value={result}/>
    </div>
    <div class="compo-item">
        <button on:click={handleEncode}>encode</button>
        <button on:click={handleDecode}>decode</button>
    </div>
</div>

<style>
    .compo {
        width: 400px;
        display: flex;
        gap: 5px;
    }
    .compo>.compo-item:first-child {
        flex-grow: 1;
    }
    .compo-item {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }
    .compo-item>textarea {
        flex-grow: 1;
        min-height: 100px;
    }
</style>