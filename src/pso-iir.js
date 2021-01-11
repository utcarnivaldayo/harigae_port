import init, { iir_design_pso } from '../wasm/pso_iir.js'

let numerator_order = 12;
let denominator_order = 8;
let pass_band_edge = 0.4;
let stop_band_edge = 0.5;
let desired_group_delay = 0.0;
let max_ripple = 0.0;
let division_approximation_band = 0;
let division_transition_band = 0;
let number_of_search_points = 0;
let max_iteration = 0;
let weight = 0.0;
let c1 = 0.0;
let c2 = 0.0;
let init_scale = 0.0;
let init_a = 0.0;
const div = 1001;
let byte_normalized_angular_frequency = new ArrayBuffer(8 * div);
let normalized_angular_frequency = new Float64Array(byte_normalized_angular_frequency);
let byte_magnitude_response = new ArrayBuffer(8 * div);
let magnitude_response = new Float64Array(byte_magnitude_response);
let byte_group_delay = new ArrayBuffer(8 * div);
let group_delay = new Float64Array(byte_group_delay);
window.chart = [];

function chartResponse(id, data_name, by, axis_data, mag_res) {

    let ctx = document.getElementById(id);
    window.chart.push(new Chart(ctx, {
        type: 'line',
        data: {
            labels: axis_data,
            datasets: [
                {
                    label: data_name,
                    data: mag_res,
                    borderColor: "rgba(2550,0,0,1)",
                    backgroundColor: "rgba(0,0,0,0)",
                    pointRadius: 0
                }
            ],
        },
        options: {
            title: {
                display: true,
                position: 'bottom',
                text: 'Normalized angular frequency'
            },
            scales: {
                xAxes: [{
                    ticks: {
                        min: 0.0,
                        max: 3.14,
                        maxTicksLimit: 10,
                    }
                }],
                yAxes: [{
                    ticks: {
                        stepSize: 5,
                        callback: function (value, index, values) {
                            return value + by
                        }
                    }
                }]
            },
            maintainAspectRatio: true
        }
    }
    )
    );
}

async function run() {

    await init();
    update();
    //chart
    chartResponse("magResponse", "Magnitude response", "dB", normalized_angular_frequency, magnitude_response);
    chartResponse("gdResponse", "Group delay", "", normalized_angular_frequency, group_delay);
}

function update() {
    //get data
    const passband = document.getElementById("passband");
    const stopband = document.getElementById("stopband");
    const numerator = document.getElementById("numerator");
    const denominator = document.getElementById("denominator");
    const tau = document.getElementById("tau");
    const delt = document.getElementById("delt");
    const diva = document.getElementById("diva");
    const divt = document.getElementById("divt");
    const individual = document.getElementById("individual");
    const ite = document.getElementById("ite");
    const wei = document.getElementById("weight");
    const pcoef = document.getElementById("pcoef");
    const gcoef = document.getElementById("gcoef");
    const iniscale = document.getElementById("iniscale");
    const ininume = document.getElementById("ininume");
    numerator_order = numerator.value;
    denominator_order = denominator.value;
    pass_band_edge = passband.value;
    stop_band_edge = stopband.value;
    desired_group_delay = tau.value;
    max_ripple = delt.value;
    division_approximation_band = diva.value;
    division_transition_band = divt.value;
    number_of_search_points = individual.value;
    max_iteration = ite.value;
    weight = wei.value;
    c1 = pcoef.value;
    c2 = gcoef.value;
    init_scale = iniscale.value;
    init_a = ininume.value;

    let byte_a = new ArrayBuffer(8 * (Number(numerator_order) + Number(1)));
    let a = new Float64Array(byte_a);
    let byte_b = new ArrayBuffer(8 * denominator_order);
    let b = new Float64Array(byte_b);

    //test
    /*
    console.log(numerator_order);
    console.log(denominator_order);
    console.log(pass_band_edge);
    console.log(stop_band_edge);
    console.log(desired_group_delay);
    console.log(max_ripple);
    console.log(division_approximation_band);
    console.log(division_transition_band);
    console.log(number_of_search_points);
    console.log(max_iteration);
    console.log(weight);
    console.log(c1);
    console.log(c2);
    console.log(init_scale);
    console.log(init_a);
    */

    const error = iir_design_pso(numerator_order, denominator_order, pass_band_edge, stop_band_edge, desired_group_delay, max_ripple, division_approximation_band, division_transition_band, number_of_search_points, max_iteration, weight, c1, c2, init_scale, init_a, normalized_angular_frequency, magnitude_response, group_delay, a, b);

    document.getElementById("error").innerText = "設計誤差 : ";
    document.getElementById("error").insertAdjacentText('beforeend', error);


    document.getElementById("result-coef-scale").innerHTML = "スケーリング係数 : <br>";
    document.getElementById("result-coef-scale").insertAdjacentText('beforeend', "a_0 : " + a[0]);

    document.getElementById("result-coef-nume").innerHTML = "分子フィルタ係数 :<br>";
    for (let i = 0; i < (a.length >>> 1); i++) {
        for (let k = 1; k <= 2; k++) {
            document.getElementById("result-coef-nume").insertAdjacentHTML('beforeend', "a_" + (Number(i) + 1) + "," + k + " : " + a[(i << 1) + k] + "<br>");
        }
    }
    if (a.length % 2 == 0) document.getElementById("result-coef-nume").insertAdjacentHTML('beforeend', "a_0,1" + " : " + a[a.length - 1] + "<br>");

    document.getElementById("result-coef-deno").innerHTML = "分母フィルタ係数 :<br>";
    for (let i = 0; i < (b.length >>> 1); i++) {
        for (let k = 1; k <= 2; k++) {
            document.getElementById("result-coef-deno").insertAdjacentHTML('beforeend', "b_" + (Number(i) + Number(1)) + "," + k + " : " + b[(i << 1) + (k - 1)] + "<br>");
        }
    }
    if (b.length % 2 == 1) document.getElementById("result-coef-deno").insertAdjacentHTML('beforeend', "b_0,1" + " : " + b[b.length - 1] + "<br>");
}

document.getElementById('btnrun').onclick = function () {

    update();
    window.chart[0].data.datasets[0].data = magnitude_response;
    window.chart[1].data.datasets[0].data = group_delay;
    window.chart[0].update();
    window.chart[1].update();
}

run();