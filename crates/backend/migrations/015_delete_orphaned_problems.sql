-- Delete problems whose subtopic no longer exists in the subtopics table
DELETE FROM problems p
WHERE NOT EXISTS (
    SELECT 1 FROM subtopics s
    WHERE s.topic_id = p.main_topic AND s.id = p.subtopic
);
