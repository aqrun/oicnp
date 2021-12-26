(function($){
    //生成从minNum到maxNum的随机数
    function randomNum(minNum, maxNum) {
        switch (arguments.length) {
            case 1:
                return parseInt(Math.random() * minNum + 1, 10);
                break;
            case 2:
                return parseInt(Math.random() * (maxNum - minNum + 1) + minNum, 10);
                //或者 Math.floor(Math.random()*( maxNum - minNum + 1 ) + minNum );
                break;
            default:
                return 0;
                break;
        }
    } 
    function changeData(peotryArray){
        var num = randomNum(0, peotryArray.length-1)
        var p = peotryArray[num]
        var $b = $('.g-banner.home-banner')
        $b.find('h2').html(p.title)
        var html = p.content.join('<br/>')
        $b.find('h3').html(html)
    }

    function peotry(){
        if(location.pathname !== '/') return;
        
        $.get('/assets/js/poetry.json',function(data){
            changeData(data)
        })
    }


    $(()=>{
        peotry()
    })
})(jQuery)