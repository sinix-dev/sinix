<template>
<div class="col-md-4 py-10 m-10">
    <div class="input-group">
        <input v-model="url" class="form-control" type="search" placeholder="Paste the Game URL" aria-label="Recipient's username" />
        <div class="input-group-append">
            <span class="input-group-text"><a :href="`${url}/`" class="searchbar-clear"><i class="fa fa-search"></i></a></span>
        </div>
    </div>
</div>
</template>

<script>
export default {
  name: "Index",
  components: {},
  // Currently I want to use this url attribute in the src of iframe but the im not sure how to pass the data to it.
  data(){
    return {
      url: ""
    }
  }
}
</script>

<style scoped>
@import "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"
</style>
