<script>
  import { Title } from "../../DesignSystem/Primitive";
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
    margin: 0 auto;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    grid-template-rows: auto;
    grid-gap: 1.5rem;
    grid-template-areas:
      "balance balance side"
      "transactions transactions transactions";
  }

  .balance,
  .send-receive,
  .transactions {
    border-radius: 4px;
    border: 1px solid var(--color-foreground-level-2);
  }

  .balance {
    grid-area: balance;
    padding: 1.25rem 1.5rem;
  }

  .send-receive {
    grid-area: side;
  }

  .transactions {
    grid-area: transactions;
  }

  .circle-chart__circle {
    animation: circle-chart-fill 2s reverse; /* 1 */
    transform: rotate(93deg); /* 2, 3 */
    transform-origin: center; /* 4 */
  }

  .circle-chart__circle--negative {
    transform: rotate(-273deg) scale(1, -1); /* 1, 2, 3 */
  }

  @keyframes circle-chart-fill {
    to {
      stroke-dasharray: 0 100;
    }
  }
</style>

<div class="container">
  <div class="balance">
    <Title style="font-size: 20px;">Balance</Title>
    <svg
      class="circle-chart"
      viewbox="0 0 33.83098862 33.83098862"
      width="200"
      height="200"
      xmlns="http://www.w3.org/2000/svg">
      <circle
        class="circle-chart__circle"
        stroke="var(--color-positive)"
        stroke-width="1"
        stroke-dasharray="84,100"
        stroke-linecap="round"
        fill="none"
        cx="16.91549431"
        cy="16.91549431"
        r="15.91549431" />
      <circle
        class="circle-chart__circle circle-chart__circle--negative"
        stroke="var(--color-foreground-level-2)"
        stroke-width="1"
        stroke-dasharray="13,100"
        stroke-linecap="round"
        fill="none"
        cx="16.91549431"
        cy="16.91549431"
        r="15.91549431" />
    </svg>

  </div>
  <div class="send-receive">hello</div>
  <div class="transactions">hello</div>
</div>
