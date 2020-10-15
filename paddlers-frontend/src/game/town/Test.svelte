<script>
    let elements = [];
    function hello() {
        alert("hello");
    }
    function addRandomElement() {
        const color =
            "#" + Math.floor(Math.random() * Math.pow(255, 3)).toString(16);
        elements.push({ color });
    }
</script>

<style>
    .random-elements {
        display: grid;
        grid-template-columns: repeat(1fr, 6);
    }
    .random-elements > {
        width: 50px;
        height: 50px;
    }
</style>

<div>
    <h1>Hello Revelte</h1>
    <button on:click={hello}>Hi!</button>
    <button on:click={addRandomElement}>Add random element</button>
    <section class="random-elements">
        {#each elements as el}
            <p style="background-color: {el.color}" />
        {/each}
    </section>
</div>
