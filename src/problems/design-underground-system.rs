// @leetup=custom
// @leetup=info id=1396 lang=rust slug=design-underground-system

/*
* An underground railway system is keeping track of customer travel times between
* different stations. They are using this data to calculate the average time it
* takes to travel from one station to another.
*
* Implement the `UndergroundSystem` class:
*
* * `void checkIn(int id, string stationName, int t)`
*
*   * A customer with a card ID equal to `id`, checks in at the station
*     `stationName` at time `t`.
*   * A customer can only be checked into one place at a time.
* * `void checkOut(int id, string stationName, int t)`
*
*   * A customer with a card ID equal to `id`, checks out from the station
*     `stationName` at time `t`.
* * `double getAverageTime(string startStation, string endStation)`
*
*   * Returns the average time it takes to travel from `startStation` to
*     `endStation`.
*   * The average time is computed from all the previous traveling times from
*     `startStation` to `endStation` that happened directly, meaning a check
*     in at `startStation` followed by a check out from `endStation`.
*   * The time it takes to travel from `startStation` to `endStation` may be
*     different from the time it takes to travel from `endStation` to
*     `startStation`.
*   * There will be at least one customer that has traveled from `startStation` to
*     `endStation` before `getAverageTime` is called.
*
* You may assume all calls to the `checkIn` and `checkOut` methods are consistent.
* If a customer checks in at time `t1` then checks out at time `t2`, then `t1 <
* t2`. All events happen in chronological order.
*
*
* Example 1:
*
* Input
* ["UndergroundSystem","checkIn","checkIn","checkIn","checkOut","checkOut","checkO
* ut","getAverageTime","getAverageTime","checkIn","getAverageTime","checkOut","get
* AverageTime"]
* [[],[45,"Leyton",3],[32,"Paradise",8],[27,"Leyton",10],[45,"Waterloo",15],[27,"W
* aterloo",20],[32,"Cambridge",22],["Paradise","Cambridge"],["Leyton","Waterloo"],
* [10,"Leyton",24],["Leyton","Waterloo"],[10,"Waterloo",38],["Leyton","Waterloo"]]
* Output
* [null,null,null,null,null,null,null,14.00000,11.00000,null,11.00000,null,12.0000
* 0]
* Explanation
* UndergroundSystem undergroundSystem = new UndergroundSystem();
* undergroundSystem.checkIn(45, "Leyton", 3);
* undergroundSystem.checkIn(32, "Paradise", 8);
* undergroundSystem.checkIn(27, "Leyton", 10);
* undergroundSystem.checkOut(45, "Waterloo", 15);  // Customer 45 "Leyton" -> "Wat
* erloo" in 15-3 = 12
* undergroundSystem.checkOut(27, "Waterloo", 20);  // Customer 27 "Leyton" -> "Wat
* erloo" in 20-10 = 10
* undergroundSystem.checkOut(32, "Cambridge", 22); // Customer 32 "Paradise" -> "C
* ambridge" in 22-8 = 14
* undergroundSystem.getAverageTime("Paradise", "Cambridge"); // return 14.00000. O
* ne trip "Paradise" -> "Cambridge", (14) / 1 = 14
* undergroundSystem.getAverageTime("Leyton", "Waterloo");    // return 11.00000. T
* wo trips "Leyton" -> "Waterloo", (10 + 12) / 2 = 11
* undergroundSystem.checkIn(10, "Leyton", 24);
* undergroundSystem.getAverageTime("Leyton", "Waterloo");    // return 11.00000
* undergroundSystem.checkOut(10, "Waterloo", 38);  // Customer 10 "Leyton" -> "Wat
* erloo" in 38-24 = 14
* undergroundSystem.getAverageTime("Leyton", "Waterloo");    // return 12.00000. T
* hree trips "Leyton" -> "Waterloo", (10 + 12 + 14) / 3 = 12
*
* Example 2:
*
* Input
* ["UndergroundSystem","checkIn","checkOut","getAverageTime","checkIn","checkOut",
* "getAverageTime","checkIn","checkOut","getAverageTime"]
* [[],[10,"Leyton",3],[10,"Paradise",8],["Leyton","Paradise"],[5,"Leyton",10],[5,"
* Paradise",16],["Leyton","Paradise"],[2,"Leyton",21],[2,"Paradise",30],["Leyton",
* "Paradise"]]
* Output
* [null,null,null,5.00000,null,null,5.50000,null,null,6.66667]
* Explanation
* UndergroundSystem undergroundSystem = new UndergroundSystem();
* undergroundSystem.checkIn(10, "Leyton", 3);
* undergroundSystem.checkOut(10, "Paradise", 8); // Customer 10 "Leyton" -> "Parad
* ise" in 8-3 = 5
* undergroundSystem.getAverageTime("Leyton", "Paradise"); // return 5.00000, (5) /
*  1 = 5
* undergroundSystem.checkIn(5, "Leyton", 10);
* undergroundSystem.checkOut(5, "Paradise", 16); // Customer 5 "Leyton" -> "Paradi
* se" in 16-10 = 6
* undergroundSystem.getAverageTime("Leyton", "Paradise"); // return 5.50000, (5 +
* 6) / 2 = 5.5
* undergroundSystem.checkIn(2, "Leyton", 21);
* undergroundSystem.checkOut(2, "Paradise", 30); // Customer 2 "Leyton" -> "Paradi
* se" in 30-21 = 9
* undergroundSystem.getAverageTime("Leyton", "Paradise"); // return 6.66667, (5 +
* 6 + 9) / 3 = 6.66667
*
*
* Constraints:
*
* * `1 <= id, t <= 106`
* * `1 <= stationName.length, startStation.length, endStation.length <= 10`
* * All strings consist of uppercase and lowercase English letters and digits.
* * There will be at most `2 * 104` calls in total to `checkIn`, `checkOut`,
*   and `getAverageTime`.
* * Answers within `10-5` of the actual value will be accepted.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
use std::collections::HashMap;

struct UndergroundSystem {
    check_ins: HashMap<i32, CheckPoint>,
    log: HashMap<String, Log>,
}

struct CheckPoint {
    station_name: String,
    time: i32,
}

struct Log {
    sum: i32,
    count: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            check_ins: HashMap::new(),
            log: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_ins.insert(
            id,
            CheckPoint {
                station_name,
                time: t,
            },
        );
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let check_in = self.check_ins.remove(&id).unwrap();
        let segment_id = UndergroundSystem::get_segment_id(check_in.station_name, station_name);
        let log = self
            .log
            .entry(segment_id)
            .or_insert(Log { sum: 0, count: 0 });
        log.count += 1;
        log.sum += t - check_in.time;
    }

    fn get_average_time(&mut self, start_station: String, end_station: String) -> f64 {
        let segment_id = UndergroundSystem::get_segment_id(start_station, end_station);
        let log = self
            .log
            .entry(segment_id)
            .or_insert(Log { sum: 0, count: 0 });
        log.sum as f64 / log.count as f64
    }

    fn get_segment_id(start_station: String, end_station: String) -> String {
        start_station + "|" + &end_station
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::UndergroundSystem;

    #[test]
    fn example_1() {
        let mut underground_system: UndergroundSystem = UndergroundSystem::new();
        underground_system.check_in(45, "Leyton".to_string(), 3);
        underground_system.check_in(32, "Paradise".to_string(), 8);
        underground_system.check_in(27, "Leyton".to_string(), 10);
        underground_system.check_out(45, "Waterloo".to_string(), 15); // Customer 45 "Leyton" -> "Waterloo" in 15-3 = 12
        underground_system.check_out(27, "Waterloo".to_string(), 20); // Customer 27 "Leyton" -> "Waterloo" in 20-10 = 10
        underground_system.check_out(32, "Cambridge".to_string(), 22); // Customer 32 "Paradise" -> "Cambridge" in 22-8 = 14
        assert_eq!(
            underground_system.get_average_time("Paradise".to_string(), "Cambridge".to_string()),
            14.00000
        ); // return 14.00000. One trip "Paradise" -> "Cambridge", (14) / 1 = 14
        assert_eq!(
            underground_system.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            11.00000
        ); // return 11.00000. Two trips "Leyton" -> "Waterloo", (10 + 12) / 2 = 11
        underground_system.check_in(10, "Leyton".to_string(), 24);
        assert_eq!(
            underground_system.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            11.00000
        ); // return 11.00000
        underground_system.check_out(10, "Waterloo".to_string(), 38); // Customer 10 "Leyton" -> "Waterloo" in 38-24 = 14
        assert_eq!(
            underground_system.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            12.00000
        );
        // return 12.00000. Three trips "Leyton" -> "Waterloo", (10 + 12 + 14) / 3 = 12
    }
}
// @leetup=inject:after_code
