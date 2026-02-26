import subprocess
import os
from datetime import datetime

def generate_report():
    report_file = "qa_report_latest.md"
    now_str = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    print("Running tests... Please wait.")
    
    # Run pytest and collect report
    result = subprocess.run(
        ["pytest", "-v", "--tb=short"], 
        capture_output=True, 
        text=True,
        cwd=os.path.dirname(os.path.abspath(__file__))
    )
    
    with open(report_file, "w") as f:
        f.write("# Termrender Python bindings QA Report\n")
        f.write(f"**Generated:** {now_str}\n\n")
        f.write("## Overview\n")
        if result.returncode == 0:
            f.write("✅ All API tests passed.\n\n")
        else:
            f.write("❌ Tests failed. See details below.\n\n")
            
        f.write("## Pytest Output\n")
        f.write("```text\n")
        f.write(result.stdout)
        f.write("\n")
        if result.stderr:
            f.write(result.stderr)
            f.write("\n")
        f.write("```\n\n")
        f.write("---\n")
        f.write("*End of report*\n")
        
    print(f"Report Generated: {os.path.abspath(report_file)}")

if __name__ == "__main__":
    generate_report()
