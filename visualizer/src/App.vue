<template>
  <div id="app">
    <button v-on:click="connect">Random Connect</button>
    <button v-on:click="reset">Reset</button>
    <button v-on:click="submit">Submit</button>
    <select v-model="algo_select">
      <option value="lee">Lee</option>
      <option value="lee_minimum_crossing">Lee with Minimum Crossing</option>
      <option value="hadlock">Hadlock</option>
      <option value="stst">STST</option>
    </select>
    <canvas ref="canvas" width="500" height="500" v-on:click="clickCanvas"></canvas>
  </div>
</template>

<script>
// https://github.com/rustwasm/wasm-pack/issues/911
import init, {Maze, CellState, Points} from 'maze-routing/maze_routing';
let mod = null;

export default {
  name: "App",
  data: () => ({
    m: 8,
    n: 8,
    maze: null,
    selected: [],
    algo_select: "lee",
    algo: null
  }),

  async mounted() {
    mod = await init();
    this.maze = new Maze(this.m, this.n);
    this.algo = this.maze.lee_mut;
    this.draw();
  },

  watch: {
    algo_select() {
      if (this.algo_select === "lee") {
        this.algo = this.maze.lee_mut;
      } else if (this.algo_select === "lee_minimum_crossing") {
        this.algo = this.maze.lee_minimum_crossing_mut;
      } else if (this.algo_select === "hadlock") {
        this.algo = this.maze.hadlock_mut;
      } else if (this.algo_select === "stst") {
        this.algo = function() {
          let arg = [];
          for (let i = 0; i < arguments.length; i += 2) {
            arg.push([arguments[i], arguments[i + 1]]);
          }
          let points = new Points(arg);
          this.stst_mut(points);
          console.log(this, arguments, points);
        };
      } else {
        this.algo = null;
      }
    }
  },

  methods: {
    getPosX(x) {
      return x * 50 + 25;
    },
    getPosY(y) {
      return (this.n - y - 1) * 50 + 25;
    },
    clickCanvas(event) {
      const canvas = this.$refs.canvas;
      const boundingRect = canvas.getBoundingClientRect();

      const scaleX = canvas.width / boundingRect.width;
      const scaleY = canvas.height / boundingRect.height;

      const canvasX = (event.clientX - boundingRect.left) * scaleX;
      const canvasY = (event.clientY - boundingRect.top) * scaleY;

      for (let x = 0; x < this.m; x++) {
        for (let y = 0; y < this.n; y++) {
          const posX = this.getPosX(x);
          const posY = this.getPosY(y);
          if (
            Math.pow(posX - canvasX, 2) + Math.pow(posY - canvasY, 2) < 80 &&
            this.maze.get(x, y) == CellState.Empty
          ) {
            this.selected.push(x);
            this.selected.push(y);
          }
        }
      }
    },
    draw() {
      if (this.$refs.canvas) {
        let ctx = this.$refs.canvas.getContext("2d");
        ctx.clearRect(0, 0, window.innerWidth, window.innerHeight);
        for (let x = 0; x < this.m; x++) {
          for (let y = 0; y < this.n; y++) {
            let cell = this.maze.get(x, y);
            let posX = this.getPosX(x);
            let posY = this.getPosY(y);
            ctx.beginPath();
            ctx.strokeStyle = "black";
            for (let i = 0; i < this.selected.length; i += 2) {
              if (x == this.selected[i] && y == this.selected[i + 1]) {
                ctx.strokeStyle = "red";
                break;
              }
            }
            if (cell === CellState.Empty) {
              ctx.arc(posX, posY, 5, 0, 360);
            } else if (cell == CellState.Blocked) {
              let crossSize = 5;
              ctx.moveTo(posX - crossSize, posY - crossSize);
              ctx.lineTo(posX + crossSize, posY + crossSize);
              ctx.moveTo(posX - crossSize, posY + crossSize);
              ctx.lineTo(posX + crossSize, posY - crossSize);
            } else if (cell == CellState.LR) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo((this.getPosX(x + 1) + posX) / 2, posY);
            } else if (cell == CellState.UD) {
              ctx.moveTo(posX, (this.getPosY(y - 1) + posY) / 2);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
            } else if (cell == CellState.LU) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
            } else if (cell == CellState.LD) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y - 1) + posY) / 2);
            } else if (cell == CellState.RU) {
              ctx.moveTo((this.getPosX(x + 1) + posX) / 2, posY);
              ctx.lineTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
            } else if (cell == CellState.RD) {
              ctx.moveTo((this.getPosX(x + 1) + posX) / 2, posY);
              ctx.lineTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y - 1) + posY) / 2);
            } else if (cell == CellState.Cross) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo((this.getPosX(x + 1) + posX) / 2, posY);
              ctx.moveTo(posX, (this.getPosY(y - 1) + posY) / 2);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
            } else if (cell == CellState.LUR) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo((this.getPosX(x + 1) + posX) / 2, posY);
              ctx.moveTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
            } else if (cell == CellState.RDL) {
              ctx.moveTo((this.getPosX(x - 1) + posX) / 2, posY);
              ctx.lineTo((this.getPosX(x + 1) + posX) / 2, posY);
              ctx.moveTo(posX, posY);
              ctx.lineTo(posX, (this.getPosY(y - 1) + posY) / 2);
            } else if (cell == CellState.URD) {
              ctx.moveTo(posX, (this.getPosY(y - 1) + posY) / 2);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
              ctx.moveTo(posX, posY);
              ctx.lineTo((this.getPosX(x + 1) + posX) / 2, posY);
            } else if (cell == CellState.DLU) {
              ctx.moveTo(posX, (this.getPosY(y - 1) + posY) / 2);
              ctx.lineTo(posX, (this.getPosY(y + 1) + posY) / 2);
              ctx.moveTo(posX, posY);
              ctx.lineTo((this.getPosX(x - 1) + posX) / 2, posY);
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
      this.algo.apply(this.maze, [x1, y1, x2, y2]);
    },
    reset() {
      this.maze = new Maze(this.m, this.n);
    },
    submit() {
      this.algo.apply(this.maze, this.selected);
      this.selected = [];
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
