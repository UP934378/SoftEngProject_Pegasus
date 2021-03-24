// https://github.com/M30819-2020/cw2-t1/blob/master/visualisations.md - visualisation list
// https://github.com/M30819-2020/cw3-t1/blob/master/build/CW3DesignDocument.pdf - ERD
// https://github.com/M30819-2020/cw-code-t1/blob/main/database_script.txt - dummy data

// https://www.w3schools.com/howto/howto_js_popup_form.asp - pop up form (use this in cell with overflow scroll)

// Every user can have different config
// UserID, ViewID, Index, X value, Y value, Where, Start date/Start time, End date/End time, type

/**
 * @overview This is the documentation of the JavaScript code for the Pagassas website
 * @author Max for T1
 * @see <a href="https://m30819-2020.github.io/cw-code-t1/">Pegassas Documentation Homepage</a>
 */


/**
 * This is the main class that generate and manage all the graphs in the website
 */
class Graph {

  /**
   * Initialize graph details
   * @param {*} cellid Graph container cell
   * @param {*} x X axis parameter
   * @param {*} y Y axis parameter
   * @param {*} table Database source
   * @param {*} start Date range start
   * @param {*} end Date range finish
   * @param {*} type Graph type
   * @param {*} where Probe or Cell
   * @param {*} equals ID of probe or cell
   */
  constructor(cellid, x, y, table, start, end, type, where, equals) {

    // Set all possible y values
    this.yvalues = {
      'cell': ['cell_voltage', 'balance_current'],
      'battery': ['battery_current', 'state_charge', 'charge_perc', 'batt_temperature'],
      'inverter': ['sol_inv_voltage', 'sol_inv_power', 'sol_inv_frequency'],
      'grid_power': ['grid_power'],
      'house_power': ['house_power'],
    }

    // Set table tags
    this.tags = {
      'cell': ['cell_id', 'probe_id'],
      'battery': ['probe_id'],
      'inverter': ['solar_id', 'probe_id'],
      'grid_power': ['probe_id'],
      'house_power': ['probe_id'],
    }

    // Loop through keys and combine lists
    // Create full values
    this.allvalues = [this.yvalues].concat([this.tags])
    console.log(this.allvalues)

    // var jsonArray1 = [{'name': "doug", 'id':5}, {'name': "dofug", 'id':23}];
    // var jsonArray2 = [{'name': "goud", 'id':1}, {'name': "doaaug", 'id':52}];
    // jsonArray1 = jsonArray1.concat(jsonArray2);

    // Set all possible graph types
    this.types = {
      'scatter': {
        'name': 'scatter',
        'type': 'scatter',
        'mode': 'markers'
      },
      'line': {
        'name': 'line',
        'type': 'scatter',
        'mode': 'lines'
      },
      'bar': {
        'name': 'bar',
        'type': 'bar'
      }
      // 'bubble': {
      //   'name': 'bubble',
      //   'type': 'scatter',
      //   'mode': 'markers',
      //   'marker': {
      //     'size': 18 // Can edit size here to be some values
      //   }
      // }
    }

    this.values = {
      'x': x,
      'y': y,
      'table': table,
      'start': start,
      'end': end,
      'type' : this.types[type],
      'where': where,
      'equals': equals
    }

    // Create graph container cell
    this.cell = this.createCell(cellid)

    
    // Get data from API and update graph
    const getdata = async () => {
      // Get response from url, wait for response before continuing
      await this.fetchData()
      
      // Format response data
      this.formatResponse()

      // Plot the graph
      this.plotData()
    }
    getdata()
  }

  // Function to create empty cell
  createCell(id) {
    // Create empty cell
    const cell = document.createElement('div')
    cell.id = id
    cell.className = "container-cell"
    cell.draggable = "true"


    // Create config form
    const configfrm = document.createElement('form')
    configfrm.classList.add("config-form")
    configfrm.classList.add("hide")
    configfrm.onsubmit="graphs[" + "'" + cell.id + "'" + "].formSubmit(this)"
    configfrm.id = id + "-config-form"

    // When submit, call function
    configfrm.addEventListener('submit', (e) => {
      e.preventDefault()
      this.formSubmit()
    })

    this.configfrm = configfrm

    // Create form header
    var header = document.createElement('h2')
    header.textContent = cell.id
    header.classList.add("form-header")
    header.style.gridColumn = "span 2"

    configfrm.appendChild(header)
    // grid-column: col-start / span 12;
    
    // Create x & y input box's
    const xinput = document.createElement('select')
    xinput.id = id + "-xinput"
    xinput.name = "xinput"
    xinput.value = this.values.x
    xinput.classList.add("form-input")
    xinput.setAttribute('form', configfrm.id)

    // Onchange listener to disable same option selection
    xinput.addEventListener('change', () => {
      this.xopt.disabled = false
      yinput.options[this.xopt.index].disabled = false

      this.xopt = xinput.options[xinput.selectedIndex]

      this.xopt.disabled = true
      yinput.options[this.xopt.index].disabled = true
    })
    
    const xlabel = document.createElement("label")
    xlabel.htmlFor = xinput.id
    xlabel.classList.add("form-label")
    xlabel.innerHTML="X value: " 


    const yinput = document.createElement('select')
    yinput.id = id + "-yinput"
    yinput.name = "yinput"
    yinput.classList.add("form-input")
    yinput.setAttribute('form', configfrm.id)

    // Onchange listener to disable same option selection
    yinput.addEventListener('change', () => {
      this.yopt.disabled = false
      xinput.options[this.yopt.index].disabled = false

      this.yopt = yinput.options[yinput.selectedIndex]

      this.yopt.disabled = true
      xinput.options[this.yopt.index].disabled = true
    })

    const ylabel = document.createElement("label")
    ylabel.htmlFor = yinput.id
    ylabel.classList.add("form-label")
    ylabel.innerHTML="Y value: " 
    
    // Add options to x & y input boxes
    for (var key in this.yvalues) {
      
      // Create breaks between options
      if (key != Object.keys(this.yvalues)[0]) {
        var opthead = document.createElement('optgroup')
        opthead.label = ""

        xinput.appendChild(opthead)
        yinput.appendChild(opthead.cloneNode(true))
      }

      // Add option headers
      var opthead = document.createElement('optgroup')
      opthead.label = key
      xinput.appendChild(opthead)
      yinput.appendChild(opthead.cloneNode(true))

      for (var value of this.yvalues[key]) {
        // Add options
        var opt = document.createElement('option')
        opt.value = value
        opt.innerHTML = value
        opt.setAttribute('table', key)

        xinput.appendChild(opt)
        yinput.appendChild(opt.cloneNode(true))
      }
    }

    // Set selected x/y values
    xinput.value = this.values.x
    yinput.value = this.values.y

    // Initialize onchange event listener
    this.xopt = xinput.options[xinput.selectedIndex]
    this.yopt = yinput.options[yinput.selectedIndex]
    
    // Trigger event at start
    var event = new Event('change')

    xinput.dispatchEvent(event)
    yinput.dispatchEvent(event)

    // Append labels & inputs to config form
    // X always time, remove append
    // configfrm.appendChild(xlabel)
    // configfrm.appendChild(xinput)

    configfrm.appendChild(ylabel)
    configfrm.appendChild(yinput)
    

    // Create time selectors
    var dateinput = []
    var timeinput = []
    for (var [index, se] of ['Start', 'End'].entries()) {
      // Get default start / end times
      if (index == 0) {
        var datevalue = this.values.start.split("T")[0]
        var timevalue = this.values.start.split("T")[1]
      } else {
        var datevalue = this.values.end.split("T")[0]
        var timevalue = this.values.end.split("T")[1]
      }

      // Date
      dateinput[se] = document.createElement('input')
      dateinput[se].type = "date"
      dateinput[se].name = se + "dateinput"
      dateinput[se].defaultValue = datevalue
      dateinput[se].id = cell.id + "-" + se + "date"
      dateinput[se].classList.add("form-input")
      dateinput[se].setAttribute('form', configfrm.id)

      const datelabel = document.createElement("label")
      datelabel.htmlFor = dateinput.id
      datelabel.classList.add("form-label")
      datelabel.innerHTML= se + ": " 

      // Time
      timeinput[se] = document.createElement('input')
      timeinput[se].type = "time"
      timeinput[se].name = se + "timeinput"
      timeinput[se].id = cell.id + "-" + se + "time"
      timeinput[se].value = timevalue
      timeinput[se].step = 1
      timeinput[se].classList.add("form-input")
      timeinput[se].setAttribute('form', configfrm.id)
      
      const timelabel = document.createElement("label")
      timelabel.htmlFor = timeinput.id
      timelabel.classList.add("form-label")
      timelabel.innerHTML="" 

      configfrm.appendChild(datelabel)
      configfrm.appendChild(dateinput[se])
      configfrm.appendChild(timelabel)
      configfrm.appendChild(timeinput[se])
    }
    
    
    // Start / End date validation
    dateinput['Start'].addEventListener('change', function() {
      if (dateinput['Start'].value > dateinput['End'].value) {
        dateinput['Start'].value = dateinput['End'].value
      }
      if (dateinput['Start'].value == dateinput['End'].value) {
        // Trigger event at start
        var event = new Event('change')

        timeinput['Start'].dispatchEvent(event)
        timeinput['End'].dispatchEvent(event)
      }
    });

    dateinput['End'].addEventListener('change', function() {
      if (dateinput['End'].value < dateinput['Start'].value) {
        dateinput['End'].value = dateinput['Start'].value
      }
      if (dateinput['End'].value == dateinput['Start'].value) {
        // Trigger event at start
        var event = new Event('change')

        timeinput['End'].dispatchEvent(event)
        timeinput['Start'].dispatchEvent(event)
      }
    });


    // Start / End time validation
    timeinput['Start'].addEventListener('change', function() {
      if (dateinput['Start'].value == dateinput['End'].value) {
        if (timeinput['Start'].value > timeinput['End'].value) {
          timeinput['Start'].value = timeinput['End'].value
        }
      }
    });

    timeinput['End'].addEventListener('change', function() {
      if (dateinput['End'].value == dateinput['Start'].value) {
        if (timeinput['End'].value < timeinput['Start'].value) {
          timeinput['End'].value = timeinput['Start'].value
        }
      }
    });



    // Create type input
    const typeinput = document.createElement('select')
    typeinput.id = id + "-typeinput"
    typeinput.name = "typeinput"
    typeinput.classList.add("form-input")
    typeinput.setAttribute('form', configfrm.id)

    const typelabel = document.createElement("label")
    typelabel.htmlFor = typeinput.id
    typelabel.classList.add("form-label")
    typelabel.innerHTML="Type: " 
    
    for (var value in this.types) {
      // Add options
      var opt = document.createElement('option')
      opt.value = value
      opt.innerHTML = value

      typeinput.appendChild(opt)
    }

    // Set selected type
    typeinput.value = this.values.type.name

    configfrm.appendChild(typelabel)
    configfrm.appendChild(typeinput)


    // Create submit button

    var input = document.createElement('input')
    input.type = "submit"
    input.value = "Submit" 
    input.classList.add("form-input")
    input.setAttribute('onclick', "graphs[" + "'" + cell.id + "'" + "].toggleForm()")
    input.setAttribute('form', configfrm.id)

    configfrm.append(input)
    // <td><input type="submit" form="probe-config-2" value="Submit"></td>


    // Append config form to cell
    this.form = configfrm
    cell.append(configfrm)


    // Create config button
    const configbtn = document.createElement('button')
    configbtn.classList.add("config-button")
    configbtn.textContent = "Configure "
    configbtn.setAttribute('onclick', "graphs[" + "'" + cell.id + "'" + "].toggleForm()")
    
    this.configbtn = configbtn
    cell.append(configbtn)
    // <button type="submit" class="btn cancel" onclick="closeForm()">Close</button>


    // Add draggable code
    cell.addEventListener('dragstart', () => {
      // When dragging starts
      cell.classList.add('dragging')
    })

    cell.addEventListener('dragend', () => {
      // When dragging ends, check if cell is over another cell
      // Get currently dragged cell and store a copy (for switching)
      const celldragging = document.querySelector('.dragging')
      
      // Create copy of data for transferring
      const dataHolder = celldragging.data

      // Get cell being dragged over (unless it's dragging over itself)
      const celldraggingover = document.querySelector('.dragging-over:not(.dragging)')

      // Switch cell plots
      // Need to re-plot graphs for proper functioning
      if (celldraggingover) {
        // Remove dragging classes
        celldraggingover.classList.remove('dragging-over')
        celldragging.classList.remove('dragging')

        // Swap cell positions
        const parent = celldragging.parentNode
        const celldragging_sibling = celldragging.nextSibling

        parent.insertBefore(celldragging, celldraggingover)
        parent.insertBefore(celldraggingover, celldragging_sibling)

        // Fix object cell references
        this.fixRefs()
        graphs[celldraggingover.id].fixRefs()

        // Need to add paramater check to plotdata
        // Re-plot data
        this.plotData(this.cell, celldraggingover.data)
        this.plotData(graphs[celldraggingover.id].getCell, dataHolder)
      }
      this.cell.classList.remove('dragging')
  })

  cell.addEventListener('dragover', e => {
    if (cell.classList.contains("dragging") == false){
        // Prevent default here to remove cannot be dragged here symbol
        e.preventDefault()

        // When something is being dragged over this cell
        cell.classList.add('dragging-over')
    }
  })

  cell.addEventListener('dragleave', () => {
      // When something is no longer being dragged over this cell
      setTimeout(() => {
        cell.classList.remove('dragging-over')
      }, 1)
      // Have to wait here otherwise dragleave executes before dragend can find dragover element
  })

    return cell
  }

  // Fetch data from API & return after response (stops response being a JS 'Promise')
  async fetchData() {
    console.log(this.values.x)
    console.log(this.values.y)
    if (this.values.x == 'time') {
      var url = "http://127.0.0.1:8085/dbapi/" + this.values.table + "?" 
      + this.values.y + "_col=true&start=" 
      + this.values.start + "Z&end=" + this.values.end + "Z&"
      + this.values.where + "=" + this.values.equals
    } else if (this.values.y == 'time') {
      var url = "http://127.0.0.1:8085/dbapi/" + this.values.table + "?" 
      + this.values.x + "_col=true&start=" 
      + this.values.start + "Z&end=" + this.values.end + "Z&"
      + this.values.where + "=" + this.values.equals
    } else {
      var url = "http://127.0.0.1:8085/dbapi/" + this.values.table + "?" 
      + this.values.x + "_col=true&" + this.values.y + "_col=true&start=" 
      + this.values.start + "Z&end=" + this.values.end + "Z&"
      + this.values.where + "=" + this.values.equals
    }
    console.log(url)

    // Possibility of X always being time, Y being the value that changes
    // Ask Zac
    
    // /dbapi/table?select_1_col=true&select_2_col&start=datetimeZ&end=datetimeZ&select_1=value
    // Function to fetch data from url
    const waitresponse = async () => {
        const data = await fetch(url)
                          .then(response => response.json())
                          .catch(error => console.error(error))
        return data
    }
  
    // Wait for response from function to assign response
    const response = await waitresponse()

    if (response) {
      // Set object response
      this.response = response

      // return response once it has been assigned
      return response; 
    } else {
      console.error("API fetch failed: No data avaliable for this timeframe?")
    }
     
  }

  // Plot some data into a cell, can add parameters like layout, config etc
  plotData(cell = this.cell, data = this.data) {
      // Graph layout options
      // See " https://plotly.com/javascript/reference/ " for individual plot type layout options
      var layout = { 
          margin: {
              l: 40,
              r: 10,
              b: 30,
              t: 10
            },
          showlegend: false,
          xaxis: {automargin: true},
          yaxis: {automargin: true}
      }
      
      // Graph configuration options
      // See " https://plotly.com/javascript/configuration-options/ " for options
      var config = {
        responsive: true
      }

      Plotly.newPlot(cell, data, layout, config)
  }

  // Re-link object references with HTML DOM elements
  fixRefs() {
    this.form = document.getElementById(this.form.id)
    this.cell = document.getElementById(this.cell.id)
  }

  // Toggle configuration form on / off
  toggleForm() {
    if (this.form.classList.contains("hide")) {
      this.form.classList.remove("hide")
      // Move button to right of cell
      this.configbtn.style.left = '70%'
      this.configbtn.textContent = "Close"
    } else {
      this.form.classList.add("hide")
      // Move button to left of cell
      this.configbtn.style.left = "0"
      this.configbtn.textContent = "Configure"
    }
    
  }
  

  // Update graph on form submit
  formSubmit() {
    // Get data from api with below values
    // Plot data returned

    // Update object values
    this.values.x = this.configfrm.xinput.value
    this.values.y = this.configfrm.yinput.value
    this.values.table = this.configfrm.xinput.options[this.configfrm.xinput.selectedIndex].attributes.table.value
    // Currently, for multi-table selection
    // Would need to define xtable & ytable and call fetchData multiple times + add paramaters to fetch table
    // How would the data join up?
    this.values.type = this.types[this.configfrm.typeinput.value]
    this.values.start = this.configfrm.Startdateinput.value + "T" + this.configfrm.Starttimeinput.value
    this.values.end = this.configfrm.Enddateinput.value + "T" + this.configfrm.Endtimeinput.value

    // Get data from API and update graph
    const getdata = async () => {
      // Get response from url, wait for response before continuing
      await this.fetchData()
      
      // Format response data
      this.formatResponse()

      // Plot the graph
      this.plotData()
    }
    getdata()
      // /dbapi/table?select_1_col=true&select_2_col&start=datetimeZ&end=datetimeZ&select_1=value
  }

  formatResponse() {
    // Function that takes response & x/y values

    // Handle if same X and Y (stop from being chosen)
    // Handle different tables
    
    const x = []
    const y = []

    switch (true && true) {
      // X is not time, Y is not time
      case (this.values.x != 'time') && (this.values.y != 'time'):
        for (var value of this.response.values) {
          x.push(value[1])
          y.push(value[2])
        }
        break

      // X is time, Y is not time
      case (this.values.x == 'time') && (this.values.y != 'time'):
        for (var value of this.response.values) {
          var xseconds = value[0]
          var xdate = new Date(0)

          xdate.setUTCSeconds(xseconds)
          
          x.push(xdate)
          y.push(value[1])
        }
        break

      // X is not time, y is time
      case (this.values.x != 'time') && (this.values.y == 'time'):
        for (var value of this.response.values) {
          var yseconds = value[0]
          var ydate = new Date(0)

          ydate.setUTCSeconds(yseconds)

          x.push(value[1])
          y.push(ydate)
        }
        break
    }

    // Create trace
    const data = {
          name: "temp",
          x: x,
          y: y,
          type: this.values.type.type
    }

    if (this.values.type.mode) {
      data['mode'] = this.values.type.mode
    } if (this.values.type.markers){
      data['marker'] = this.values.type.markers
    }

    console.log(data)
    // Set graph object data
    this.data = [data]
  }

  // Return graph info
  get getCell() {
    return this.cell
  }

  get getData() {
    return this.data
  }

}







// mac_address MACADDR PRIMARY KEY,
// request_interval INT NOT NULL,

// https://github.com/M30819-2020/cw-code-t1/blob/main/postgres_script.txt

// Row object for each probe option
class ProbeOption {
  // Initialize graph details with paramaters
  constructor(probeID, probeInterval) {
      // Set option ID
      this.id = probeID

      // Set probe frequency
      this.data = {
        notifcations: !!probeInterval, // If probe interval 0: false, greater than 0: true
        frequency: probeInterval
      }

      // Create empty row
      this.row = this.createRow()
  }


  createRow() {
      // Create empty row
      const row = document.createElement('div')
      row.classList.add("div-table-row")
      row.name = this.id
      row.id = this.id


      // Create form
      const formCell = document.createElement('div')
      formCell.classList.add("div-table-cell")
      formCell.id = this.id + "-form-cell"

      const form = document.createElement('form')
      form.id = this.id + "-form"
      form.action = "/config.html"
      form.method = "POST"

      var input = document.createElement('input')
      input.type = "hidden"
      input.name = "id"
      input.value = this.id

      form.append(input)
      formCell.append(form)
      // <td><form id="probe-1"><input type="hidden" name="id" value="probe-1"></form></td>
      

      // Create name cell
      const nameCell = document.createElement('div')
      nameCell.classList.add("div-table-cell")
      nameCell.id = this.id + "-name-cell"

      const paragraph = document.createElement("p")
      paragraph.textContent = this.id

      nameCell.append(paragraph)
      nameCell.append(paragraph)
      // <td><p>Probe 2</p></td>


      // Create frequency cell
      // Have to create frequency cell before data cell because data cell references frequency cell
      const freqCell = document.createElement('div')
      freqCell.classList.add("div-table-cell")
      freqCell.id = this.id + "-freq-cell"

      var input = document.createElement('input')
      input.type = "number"
      input.value = this.data.frequency
      input.setAttribute('form', this.id + "-form")
      input.name = "frequency"

      // Cannot go below 1 min or above 60 min
      input.min = 1
      input.max = 60
      

      this.freq = freqCell
      freqCell.append(input)
      // <td><input id="frequency-2" type="number" form="probe-config-2" name="frequency" value="0"></td>


      // Create data cell
      const dataCell = document.createElement('div')
      dataCell.classList.add("div-table-cell")
      dataCell.id = this.id + "-data-cell"

      var input = document.createElement('input')
      input.type = "checkbox"
      input.checked = this.data.notifcations
      input.setAttribute('form', this.id + "-form")
      input.name = "data"
      input.setAttribute('onchange', "rows[this.parentNode.parentNode.id].toggleFrequency(this.checked)")
      this.toggleFrequency(input.checked)

      dataCell.append(input)
      // <td><input type="checkbox" form="probe-config-2" name="data" onchange="toggleDisabled(this.checked)"></td>


      // Create submit cell
      const submitCell = document.createElement('div')
      submitCell.classList.add("div-table-cell")

      var input = document.createElement('input')
      input.type = "submit"
      input.value = "Submit"
      input.setAttribute('form', this.id + "-form")

      submitCell.append(input)
      // <td><input type="submit" form="probe-config-2" value="Submit"></td>

      // Append cells to row
      row.append(formCell)
      row.append(nameCell)
      row.append(dataCell)
      row.append(freqCell)
      row.append(submitCell)

      // Return filled in row
      return(row)
  }

  // Toggle frequency cell on/off
  toggleFrequency(checked) {
      if (checked) {
          this.freq.children[0].disabled = false
      } else {
          this.freq.children[0].disabled = true
      }
  }

  // return whole row
  get getRow() {
      return this.row
  }

  // return form id
  get formCell() {
      return(this.row.children[0].children[0].id)
  }

  // return name
  get nameCell() {
      return(this.row.children[1].children[0].textContent)
  }

  // return data checked (true/false)
  get dataCell() {
      return(this.row.children[2].children[0].checked)
  }

  // return frequency
  get freqCell() {
      return(this.row.children[3].children[0].value)
  }
}



// Insert HTML DOM into a parent element (can specify element to be inserted before by id)
function insertDOM(parent, DOM, before = null) {

  // Get parent container
  var parent = document.getElementById(parent)
  
  // Append empty cell to parent
    const lastElement = parent.lastElementChild
    // If last element exists & needs to go before an element
    if (lastElement && before) {
      // If the last element of parent is a 'last-element', insert before it
        if (lastElement.classList.contains(before)) {
          parent.insertBefore(DOM, lastElement)
        } else {
          // Otherwise append
          parent.appendChild(DOM)
        }
    } else {
      // Otherwise append
      parent.appendChild(DOM)
    }

}



// If on home page
if (window.location.pathname === '/home.html') {

  // Create 11 of the same graphs
  var graphs = []
  for (var i = 0;i <= 11;i++) {
    // Create graph with some set data
    graphs[i] = new Graph(('container' + i), 'time', 'cell_voltage', 'cell', 
    '2021-03-01T00:00:00', '2021-04-01T00:00:00', 
    'line', 'cell_id', '1')
    

    // getData doesn't work because graph.data is set in async function, put decleration in async function?
    // Could use a init(callback) in the class to wait for the response before calling the function
    

    // Insert graph into content container before last-element (if it exists)
    insertDOM('content-container', graphs[i].getCell, 'last-element')
  }

}


// If on visualisation page
if (window.location.pathname === '/visualisations.html') {

  // Create 5 of the same graphs
  var graphs = []
  for (var i = 0;i <= 5;i++) {
    // Create graph with some set data
    // /dbapi/table?select_1_col=true&select_2_col&start=2021-03-10T00:00:00Z&end=datetimeZ&select_1=value
    // /dbapi/cell?cell_id_col=true&cell_voltage_col=true&start=2021-03-01T00:00:00Z&end=2021-04-01T00:00:00Z&cell_id=1
    graphs['container' + i] = new Graph(('container' + i), 'time', 'cell_voltage', 'cell', 
                                                          '2021-03-01T00:00:00', '2021-04-01T00:00:00', 
                                                          'line', 'cell_id', '1')

    // Insert graph into content container before last-element (if it exists)
    insertDOM('content-container', graphs['container' + i].getCell, 'last-element')
  }



  // Create add new graph button
  var newgraphbtn = document.createElement("a")
  newgraphbtn.classList.add('last-element')
  newgraphbtn.id = 'add-container-cell'
  // When cell is clicked
  newgraphbtn.addEventListener('click',() => {
    // Create new graph   
    graphs['container' + Object.keys(graphs).length] = new Graph(('container' + i), 'time', 'cell_voltage', 'cell', 
                                                                  '2021-03-01T00:00:00', '2021-04-01T00:00:00', 
                                                                  'line', 'cell_id', '1')
    
    insertDOM('content-container', graphs['container' + (Object.keys(graphs).length - 1)].getCell, 'last-element')
  })
  // Insert add button image to svg
  newgraphbtn.innerHTML = "<img src='images/add.svg' id='add-button'>"
  // Append to container
  insertDOM("content-container", newgraphbtn)


  // Create add new tab button
  var newtabbtn = document.createElement("a")
  newtabbtn.classList.add('add-tab')
  newtabbtn.id = 'add-tab'
  // When cell is clicked
  newtabbtn.addEventListener('click',() => { 
      // Create new tab will choose default values
      let tab = document.createElement("a");
      tab.href = "";
      tab.classList.add("tab");
      tab.innerHTML = "New Tab";
      insertDOM("tab-bar", tab, "add-tab")
  })
  // Insert add button image to svg
  newtabbtn.innerHTML = "<img src='images/add.svg'>"
  // Append to container
  insertDOM("tab-bar", newtabbtn)


  
  // Create tabs with some generic names
  for (var i = 1;i <= 10;i++) {
    let tab = document.createElement("a");
    tab.href = "";
    tab.classList.add("tab");
    tab.innerHTML = "Tab " + i;
    insertDOM("tab-bar", tab, "add-tab")
  }
}


// Have JS generate table row here with columns: probe_id, recieve notifcation checkbox, notifcation frequency input / number box

// If on configuration page
if (window.location.pathname === '/config.html') {

  var rows = []

  const getProbe = async () => {
    // Function to fetch data from url
    
    // Get probes from getprobe function
    const data = await fetch("http://127.0.0.1:8085/getprobes")
                      .then(response => response.json())
                      .catch(error => console.error(error))
    
    for (probe of data) {
      rows[probe.mac_address] = new ProbeOption(probe.mac_address, probe.request_interval)

      // Insert graph into content container before last-element (if it exists)
      insertDOM('probe-table', rows[probe.mac_address].getRow)
    }
  }
  getProbe()

}