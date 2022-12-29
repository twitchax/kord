//var helpers = {};

export function createStaff(elementId, width, height, clef, notes) {
    document.getElementById(elementId).innerHTML = '';
    const vf = new Vex.Flow.Factory({
        renderer: { elementId, width, height },
    });
      
    const score = vf.EasyScore();
    const system = vf.System();
      
    system
        .addStave({
            voices: notes.map(n => score.voice(score.notes(`${n}/w`)),),
        })
        .addClef(clef)
        .addTimeSignature('4/4');
    
    vf.draw();
};

//window.helpers = helpers;