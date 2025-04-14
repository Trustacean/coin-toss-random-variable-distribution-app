<script lang="ts">
    import { onMount } from 'svelte';
    import * as d3 from 'd3';
  
    export let distribution: number[] = [];
  
    let width = 700;
    let height = 300;
    let margin = { top: 20, right: 20, bottom: 40, left: 40 };
  
    let svg: SVGSVGElement;
  
    onMount(async () => {
      drawChart();
    });
  
    $: {
      if (distribution.length > 0) {
        drawChart();
      }
    }
  
    function drawChart() {
      if (!svg) return;
  
      // Clear previous chart
      d3.select(svg).selectAll('*').remove();
  
      const innerWidth = width - margin.left - margin.right;
      const innerHeight = height - margin.top - margin.bottom;
  
      const x = d3.scaleBand()
        .domain(Array.from({ length: distribution.length }, (_, i) => i.toString()))
        .range([0, innerWidth])
        .padding(0.1);
  
      const y = d3.scaleLinear()
        .domain([0, d3.max(distribution) || 1])
        .range([innerHeight, 0])
        .nice();
  
      const g = d3.select(svg)
        .attr('width', width)
        .attr('height', height)
        .append('g')
        .attr('transform', `translate(${margin.left},${margin.top})`);
  
      // Add X axis
      g.append('g')
        .attr('transform', `translate(0,${innerHeight})`)
        .call(d3.axisBottom(x))
        .append('text')
        .attr('x', innerWidth / 2)
        .attr('y', 30)
        .attr('fill', 'currentColor')
        .text('Number of Heads');
  
      // Add Y axis
      g.append('g')
        .call(d3.axisLeft(y).ticks(5))
        .append('text')
        .attr('transform', 'rotate(-90)')
        .attr('y', -30)
        .attr('dy', '0.71em')
        .attr('fill', 'currentColor')
        .text('Probability');
  
      // Add bars with hover effects
      g.selectAll('.bar')
        .data(distribution)
        .enter()
        .append('rect')
        .attr('class', 'bar')
        .attr('x', (_, i) => x(i.toString()) || 0)
        .attr('y', d => y(d))
        .attr('width', x.bandwidth())
        .attr('height', d => innerHeight - y(d))
        .attr('fill', 'steelblue')
        .attr('rx', 3) 
        .attr('ry', 3) 
        .style('transition', 'all 0.2s ease') 
        .on('mouseover', function() {
          d3.select(this)
            .attr('fill', '#6fafdf');
        })
        .on('mouseout', function() {
          d3.select(this)
            .attr('fill', 'steelblue');
        })
        .append('title')
        .text((d, i) => `Heads: ${i}\nProbability: ${(d * 100).toFixed(2)}%`);
    }
  </script>
  
  <div class="chart-container">
    <svg bind:this={svg} />
  </div>
  
  <style>
    .chart-container {
      margin: 1rem 0;
    }
  </style>