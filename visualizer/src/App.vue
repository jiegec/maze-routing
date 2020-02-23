<template>
  <div id="app">
    <button v-on:click="connect">Connect</button>
    <button v-on:click="reset">Reset</button>
    <canvas ref="canvas" width="500" height="500"></canvas>
  </div>
</template>

<script>
let mod = null;

export default {
  name: "App",
  data: () => ({
    m: 8,
    n: 8,
    maze: null
  }),

  async mounted() {
    mod = await import("maze-routing");
    this.maze = new mod.Maze(this.m, this.n);
    this.draw();
  },

  methods: {
    getPosX(x) {
      return x * 50 + 25;
    },
    getPosY(y) {
      return (this.n - y - 1) * 50 + 25;
    },
    draw() {
      if (this.$refs.canvas) {
        let ctx = this.$refs.canvas.getContext("2d");
        ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);
        ctx.strokeStyle = "black";
        for (let x = 0; x < this.m; x++) {
          for (let y = 0; y < this.n; y++) {
            let cell = this.maze.get(x, y);
            let posX = this.getPosX(x);
            let posY = this.getPosY(y);
            ctx.beginPath();
            if (cell === mod.CellState.Empty) {
              ctx.arc(posX, posY, 5, 0, 360);
            } else if (cell == mod.CellState.Blocked) {
              let crossSize = 5;
              ctx.moveTo(posX - crossSize, posY - crossSize);
              ctx.lineTo(posX + crossSize, posY + crossSize);
              ctx.moveTo(posX - crossSize, posY + crossSize);
              ctx.lineTo(posX + crossSize, posY - crossSize);
            } else if (cell == mod.CellState.LR) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo((this.getPosX(x + 1) + posX) / 2, posY);
            } else if (cell == mod.CellState.UD) {
              ctx.moveTo(posX, (this.getPosY(y - 1) + posY) / 2);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
            } else if (cell == mod.CellState.LU) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
            } else if (cell == mod.CellState.LD) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y - 1) + posY) / 2);
            } else if (cell == mod.CellState.RU) {
              ctx.moveTo((this.getPosX(x + 1) + posX) / 2, posY);
              ctx.lineTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
            } else if (cell == mod.CellState.RD) {
              ctx.moveTo((this.getPosX(x + 1) + posX) / 2, posY);
              ctx.lineTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y - 1) + posY) / 2);
            } else if (cell == mod.CellState.Cross) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo((this.getPosX(x + 1) + posX) / 2, posY);
              ctx.moveTo(posX, (this.getPosY(y - 1) + posY) / 2);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
            }
            ctx.stroke();
          }
        }
      }
      requestAnimationFrame(this.draw);
    },
    connect() {
      let x1 = Math.floor(Math.random() * Math.floor(this.m));
      let x2 = Math.floor(Math.random() * Math.floor(this.m));
      let y1 = Math.floor(Math.random() * Math.floor(this.n));
      let y2 = Math.floor(Math.random() * Math.floor(this.n));
      this.maze.lee(x1, y1, x2, y2);
    },
    reset() {
      this.maze = new mod.Maze(this.m, this.n);
    }
  }
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
