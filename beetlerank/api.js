const express = require('express');
const router = express.Router();

router.use((req, res, next) => {
  next()
})

router.get('/', (req, res) => {
  succeed(res)
});

router.get('/info', (req, res) => {
  succeed(res)
});

router.get('/uploads/checkpoints/', (req, res) => {
  let csv = `STEP,STEPNAME,X,Y,Z
0,start,0.0,1.0,2.0
-1,reset,1.0,2.0,3.0
1,*,2.0,3.0,4.0
2,end,3.0,4.0,5.0`

res.send(csv)
})

router.get('/top3/:guildhall', async function(req, res) {
  guildhall = req.params.guildhall
  if (guildhall === "DEV") {
    succeed(res)
  } else {
    fail(res)
  }
})

router.get('/top3/:guildhall/:user', async function(req, res) {
  guildhall = req.params.guildhall
  user = req.params.user

  if (guildhall === "DEV" && user === "Test User") {
    res.json({
      "ranking": [
          {
              "pos": 1,
              "time": "01:00,000",
              "name": "First",
              "realtime": "60.0",
              "date": "2022-09-18 02:21:16",
              "map": "TYRIA GENDARRAN",
              "file": "test.csv"
          },
          {
            "pos": 2,
            "time": "01:00,000",
            "name": "Second",
            "realtime": "60.0",
            "date": "2022-09-18 02:21:16",
            "map": "TYRIA GENDARRAN",
            "file": "test.csv"
        },
          {
            "pos": 3,
            "time": "01:00,000",
            "name": "Third",
            "realtime": "60.0",
            "date": "2022-09-18 02:21:16",
            "map": "TYRIA GENDARRAN",
            "file": "test.csv"
        }
      ],
      "you": [
          {
            "pos": 71,
            "time": "01:00,000",
            "name": "Seventy-First",
            "realtime": "60.0",
            "date": "2022-09-18 02:21:16",
            "map": "TYRIA GENDARRAN",
            "file": "test.csv"
        },
          {
            "pos": 72,
            "time": "01:00,000",
            "name": "Seventy-Second",
            "realtime": "60.0",
            "date": "2022-09-18 02:21:16",
            "map": "TYRIA GENDARRAN",
            "file": "test.csv"
        },
          {
            "pos": 73,
            "time": "01:00,000",
            "name": "Seventy-Third",
            "realtime": "60.0",
            "date": "2022-09-18 02:21:16",
            "map": "TYRIA GENDARRAN",
            "file": "test.csv"
        }
      ]
  })
  }
})

router.get('/cups', (req, res) => {
  res.json({
    cups: [
      "TYRIA CUP",
      "GUILDHALL CUP"
    ]
  })
})

router.get('/maps/:cup', (req, res) => {
  cup = req.params.cup
  if (cup === "TYRIA CUP") {
    res.json({
      maps: [
        "TYRIA GENDARRAN"
      ]
    })
  }
})

function succeed(res) {
  res.json({
    succeed: true
  })
}

function fail(res) {
  res.json({
    succeed: false
  })
}
module.exports = router
