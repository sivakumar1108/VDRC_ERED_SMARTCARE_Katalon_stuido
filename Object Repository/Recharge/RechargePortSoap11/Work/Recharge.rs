<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Recharge</name>
   <tag></tag>
   <elementGuidId>47ec9d8d-81a9-4847-b29f-9a5980bdd32d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:cbs=&quot;http://oss.huawei.com/business/intf/webservice/cbs&quot; xmlns:msg=&quot;http://oss.huawei.com/business/intf/webservice/cbs/msg&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;cbs:WorkOrder>
         &lt;WorkOrderRequest>
            &lt;msg:WorkOrderType>142&lt;/msg:WorkOrderType>
            &lt;msg:SubscriberNo>987654321&lt;/msg:SubscriberNo>
            &lt;msg:operationCode>uvs&lt;/msg:operationCode>
            &lt;msg:password>**a****&lt;/msg:password>
            &lt;msg:AccessMode>8&lt;/msg:AccessMode>
            &lt;msg:SerialNo>580101&lt;/msg:SerialNo>
            &lt;msg:ParaList>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>TansactionType&lt;/msg:ParaName>
                  &lt;msg:ParaValue>1&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>CurrentPPSBalance&lt;/msg:ParaName>
                  &lt;msg:ParaValue>1200000&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>TradeTime&lt;/msg:ParaName>
                  &lt;msg:ParaValue>20200602101859&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>SubCosID&lt;/msg:ParaName>
                  &lt;msg:ParaValue>40817&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>RechargeAmount&lt;/msg:ParaName>
                  &lt;msg:ParaValue>500000&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>LoanFlag&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>AccessMode&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
            &lt;msg:ParaItem>
                  &lt;msg:ParaName>SourceIP&lt;/msg:ParaName>
                  &lt;msg:ParaValue>10.245.97.82&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>              
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>AccountType&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>PreviousPOSBalance&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>CurrentPOSBalance&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>LoanAmount&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
            &lt;/msg:ParaList>
         &lt;/WorkOrderRequest>
      &lt;/cbs:WorkOrder>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope>
</soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${Recharge_url}</soapServiceEndpoint>
   <soapServiceFunction>Work</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Recharge_Url</defaultValue>
      <description></description>
      <id>842636f7-cbce-408e-aa7b-cd7b18d374bc</id>
      <masked>false</masked>
      <name>Recharge_url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/C:/Softwares/76/Recharge/recharge.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
