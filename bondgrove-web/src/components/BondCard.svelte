<script>
  export let imgUrl = '';
  export let description = '';
</script>

<div class="card bg-[#C9DB92]">
  <img class="card-img" src={imgUrl} alt={description} />
  <div class="card-body"> 
    <p class="card-description font-semibold">{description}</p>
  </div>
</div>

<style>
  .card {
    width: 300px; 
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    overflow: hidden;
  }

  .card-img {
    width: 100%;
    height: 200px;
    object-fit: cover;
  }

  .card-body {
    padding: 16px;
  }

  .card-description {
    font-size: 1rem;
    margin: 0;
  }
</style>