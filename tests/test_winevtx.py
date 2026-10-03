import os
import unittest
import winevtx

class TestWinevtxLive(unittest.TestCase):
    def test_query_events_record(self):
        events = winevtx.query_events(channel="System", limit=5)
        self.assertGreaterEqual(len(events), 1)
        ev = events[0]
        self.assertIsInstance(ev, winevtx.EventRecord)
        self.assertIsNotNone(ev.event_id)
        self.assertEqual(ev.channel, "System")
        self.assertIsInstance(ev.xml, str)
        self.assertTrue(ev.xml.startswith("<Event"))
        
        # Test dictionary conversion
        d = ev.to_dict()
        self.assertIsInstance(d, dict)
        self.assertIn("Event", d)
        
        # Test properties
        self.assertIsNotNone(ev.system)
        self.assertIn("System", ev)
        self.assertEqual(ev["System"]["Channel"], "System")

    def test_query_events_xml_format(self):
        events = winevtx.query_events(channel="System", limit=3, format="xml")
        self.assertGreaterEqual(len(events), 1)
        for xml in events:
            self.assertIsInstance(xml, str)
            self.assertTrue(xml.startswith("<Event"))

    def test_query_events_dict_format(self):
        events = winevtx.query_events(channel="System", limit=3, format="dict")
        self.assertGreaterEqual(len(events), 1)
        for d in events:
            self.assertIsInstance(d, dict)
            self.assertIn("Event", d)

    def test_query_events_json_format(self):
        events = winevtx.query_events(channel="System", limit=3, format="json")
        self.assertGreaterEqual(len(events), 1)
        for j in events:
            self.assertIsInstance(j, str)
            self.assertTrue(j.startswith("{") and "Event" in j)

    def test_iter_events(self):
        it = winevtx.iter_events(channel="System", reverse=True)
        count = 0
        for ev in it:
            self.assertIsInstance(ev, winevtx.EventRecord)
            count += 1
            if count >= 10:
                break
        self.assertEqual(count, 10)

    def test_xpath_query(self):
        # Query for Level 1, 2, 3 or 4
        query = "*[System[(Level <= 4)]]"
        events = winevtx.query_events(channel="System", query=query, limit=5)
        self.assertGreaterEqual(len(events), 1)
        for ev in events:
            self.assertIsNotNone(ev.level)
            assert ev.level is not None
            self.assertLessEqual(ev.level, 4)

    def test_offline_evtx(self):
        tmp_evtx = os.path.expandvars(r"%TEMP%\test_sys.evtx")
        if os.path.exists(tmp_evtx):
            # Test read_evtx with record format
            events = winevtx.read_evtx(tmp_evtx, limit=5)
            self.assertEqual(len(events), 5)
            ev = events[0]
            self.assertIsInstance(ev, winevtx.EventRecord)
            self.assertIsNotNone(ev.event_id)
            self.assertEqual(ev.channel, "System")
            
            # Test XML format
            xmls = winevtx.read_evtx(tmp_evtx, limit=2, format="xml")
            self.assertEqual(len(xmls), 2)
            self.assertIn("<Event", xmls[0])
            
            # Test dict format
            dicts = winevtx.read_evtx(tmp_evtx, limit=2, format="dict")
            self.assertEqual(len(dicts), 2)
            self.assertIn("Event", dicts[0])
            
            # Test EvtxFile context manager
            with winevtx.EvtxFile(tmp_evtx) as f:
                recs = f.read(limit=3)
                self.assertEqual(len(recs), 3)

    def test_xml_utilities(self):
        sample_xml = """<Event xmlns="http://schemas.microsoft.com/win/2004/08/events/event">
            <System>
                <Provider Name="TestProvider"/>
                <EventID>999</EventID>
                <Channel>Application</Channel>
            </System>
            <EventData>
                <Data Name="Param1">Hello</Data>
                <Data Name="Param2">World</Data>
            </EventData>
        </Event>"""

        d = winevtx.xml_to_dict(sample_xml)
        self.assertIsInstance(d, dict)
        self.assertEqual(d["Event"]["System"]["EventID"], 999)
        self.assertEqual(d["Event"]["EventData"]["Param1"], "Hello")
        self.assertEqual(d["Event"]["EventData"]["Param2"], "World")

        j = winevtx.xml_to_json(sample_xml)
        self.assertIsInstance(j, str)
        self.assertIn('"Param1":"Hello"', j)

    def test_package_structure(self):
        self.assertTrue(hasattr(winevtx, "__version__"))
        self.assertTrue(hasattr(winevtx, "_winevtx"))
        self.assertIn("query_events", winevtx.__all__)
        self.assertIn("iter_events", winevtx.__all__)
        self.assertIn("read_evtx", winevtx.__all__)
        self.assertIn("iter_evtx", winevtx.__all__)


if __name__ == "__main__":
    unittest.main()
